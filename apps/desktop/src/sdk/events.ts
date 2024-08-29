import { logger } from '$lib/logger';
import { listen, type Event as TauriEvent, type UnlistenFn } from '@tauri-apps/api/event';

export enum AppEventName {
	SyncUnavailable = 'sync_unavailable',
	SyncAvailable = 'sync_available',
	SyncCollectingChanges = 'sync_collecting_changes',
	SyncCollectingChangesFailed = 'sync_collecting_changes_failed',
	SyncStarted = 'sync_started',
	SyncProgress = 'sync_progress',
	SyncSleep = 'sync_sleep',
	SyncUploadFailed = 'sync_upload_failed'
}

export interface AppEventPayloads {
	[AppEventName.SyncUnavailable]: string;
	[AppEventName.SyncAvailable]: never;
	[AppEventName.SyncCollectingChanges]: never;
	[AppEventName.SyncCollectingChangesFailed]: string;
	[AppEventName.SyncStarted]: number;
	[AppEventName.SyncProgress]: number;
	[AppEventName.SyncSleep]: never;
	[AppEventName.SyncUploadFailed]: string;
}

export interface AppEvent<T> extends TauriEvent<T> {
	/** Event name */
	event: AppEventName;
}

export type ListenerCallback<T> = (event: AppEvent<T>) => void;

const LOG_TARGET = 'AppEventHandler';

export class AppEventHandler {
	unlistenHooks: UnlistenFn[] = [];

	constructor() {}

	async addListener<K extends AppEventName>(
		event: K,
		callback: ListenerCallback<AppEventPayloads[K]>
	) {
		logger.debug(LOG_TARGET, `Adding listener for event: ${event}`);

		const unlisten = await listen(event, (payload) => {
			logger.debug(LOG_TARGET, `Received event: ${event} with payload: ${JSON.stringify(payload)}`);
			callback(payload as AppEvent<AppEventPayloads[K]>);
		});

		this.unlistenHooks.push(unlisten);
	}

	destroy() {
		logger.debug(LOG_TARGET, 'Cleaning up app event listeners');
		for (const unlisten of this.unlistenHooks) {
			unlisten();
		}
	}
}
