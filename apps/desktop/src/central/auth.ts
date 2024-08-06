import { fetch, Body } from '@tauri-apps/api/http';
import type { CentralClient } from 'central';
import type { ScopeAuth } from 'surrealdb.js';
import type Surreal from 'surrealdb.js';
import { jwtDecode } from 'jwt-decode';
import type { SurrealDBToken } from 'common';
import { logger } from '$lib/logger';
import { isSurrealConnectionError } from 'common/surreal';

export enum SendSigninCodeStatus {
	TargetNotOnWhatsApp = 'target_not_on_whatsapp',
	WhatsAppError = 'whatsapp_error',
	MessageSent = 'message_sent'
}

export interface SendSigninCodeResponse {
	status: SendSigninCodeStatus;
}

export interface User {
	phone_number: string;
}

const LOG_TARGET = 'CentralAuthController';

export enum SigninError {
	InvalidCode = 'invalid_code',
	ConnectionError = 'connection_error'
}

export class CentralAuthController {
	db: Surreal;

	private _userId: string | undefined;

	constructor(private client: CentralClient) {
		this.db = client.db;
	}

	userId(): string {
		logger.debug(LOG_TARGET, 'Getting user ID');

		if (!this._userId) {
			logger.error(LOG_TARGET, 'User is not signed in');
			throw new Error('User is not signed in');
		} else {
			return this._userId;
		}
	}

	isSignedIn() {
		logger.debug(LOG_TARGET, 'Checking if user is signed in');
		return this._userId !== undefined;
	}

	async signin(phoneNumber: string, code: string): Promise<void> {
		logger.info(LOG_TARGET, `Signing in remotely for user ${phoneNumber}`);
		const token = await this.db
			.signin({
				namespace: 'magmooty',
				database: 'magmooty',
				scope: 'tutor',
				phone_number: phoneNumber,
				code
			} as ScopeAuth)
			.catch((error) => {
				if (error.message.includes('No record was returned')) {
					logger.warn(LOG_TARGET, `Invalid sign in code for ${phoneNumber}. ${error}`);
					throw new Error(SigninError.InvalidCode);
				}

				if (isSurrealConnectionError(error)) {
					throw new Error(SigninError.ConnectionError);
				}

				throw error;
			});

		logger.debug(LOG_TARGET, 'Decoding token');
		const decoded: SurrealDBToken = jwtDecode(token);

		logger.debug(LOG_TARGET, 'Setting user id');
		this._userId = decoded.ID;
	}

	async sendSigninCode(phoneNumber: string): Promise<SendSigninCodeResponse> {
		logger.info(LOG_TARGET, `Requesting signin code for ${phoneNumber}`);
		const response = await fetch(`${this.client.apiBaseUrl}/auth/send_signin_code`, {
			method: 'POST',
			body: Body.json({ phone_number: phoneNumber })
		});

		logger.debug(LOG_TARGET, `Received response: ${response.status}`);
		return response.data as SendSigninCodeResponse;
	}
}
