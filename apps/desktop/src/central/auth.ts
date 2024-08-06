import { fetch, Body } from '@tauri-apps/api/http';
import type { CentralClient } from 'central';
import type { ScopeAuth } from 'surrealdb.js';
import type Surreal from 'surrealdb.js';

export enum SendSigninCodeStatus {
	TargetNotOnWhatsApp = 'target_not_on_whatsapp',
	WhatsAppError = 'whatsapp_error',
	MessageSent = 'message_sent'
}

export interface SendSigninCodeResponse {
	status: SendSigninCodeStatus;
}

export class CentralClientAuth {
	db: Surreal;

	constructor(private client: CentralClient) {
		this.db = client.db;
	}

	async signin(phoneNumber: string, code: string): Promise<void> {
		await this.db.signin({
			namespace: 'magmooty',
			database: 'magmooty',
			scope: 'tutor',
			phone_number: phoneNumber,
			code
		} as ScopeAuth);
	}

	async sendSigninCode(phoneNumber: string): Promise<SendSigninCodeResponse> {
		const response = await fetch(`${this.client.apiBaseUrl}/auth/send_signin_code`, {
			method: 'POST',
			body: Body.json({ phone_number: phoneNumber })
		});

		return response.data as SendSigninCodeResponse;
	}
}
