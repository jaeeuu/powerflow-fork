// Automatically generated. Do not edit.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";
import * as __TAURI_EVENT from "@tauri-apps/api/event";

/** Commands */
export const commands = {
	openApp: () => typedError<null, string>(__TAURI_INVOKE("open_app")),
	isMainWindowHidden: () => __TAURI_INVOKE<boolean>("is_main_window_hidden"),
	openSettings: () => typedError<null, string>(__TAURI_INVOKE("open_settings")),
	getDeviceName: (id: string) => __TAURI_INVOKE<[string, InterfaceType[]] | null>("get_device_name", { id }),
	getMacName: () => __TAURI_INVOKE<string | null>("get_mac_name"),
	switchTheme: (theme: Theme) => __TAURI_INVOKE<void>("switch_theme", { theme }),
	getDetailById: (id: number) => typedError<ChargingHistoryDetail, string>(__TAURI_INVOKE("get_detail_by_id", { id })),
	getAllChargingHistory: () => typedError<ChargingHistory[], string>(__TAURI_INVOKE("get_all_charging_history")),
	deleteHistoryById: (id: number) => typedError<number, string>(__TAURI_INVOKE("delete_history_by_id", { id })),
	getBatteryHealthHistory: () => typedError<BatteryHealthSnapshot[], string>(__TAURI_INVOKE("get_battery_health_history")),
	getProcessEnergy: () => __TAURI_INVOKE<ProcessEnergy[]>("get_process_energy"),
	exportHistoryById: (id: number, path: string) => typedError<null, string>(__TAURI_INVOKE("export_history_by_id", { id, path })),
};

/** Events */
export const events = {
	deviceEvent: makeEvent<DeviceEvent>("device-event"),
	devicePowerTickEvent: makeEvent<DevicePowerTickEvent>("device-power-tick-event"),
	historyRecordedEvent: makeEvent<HistoryRecordedEvent>("history-recorded-event"),
	powerTickEvent: makeEvent<PowerTickEvent>("power-tick-event"),
	powerUpdatedEvent: makeEvent<PowerUpdatedEvent>("power-updated-event"),
	preferenceEvent: makeEvent<PreferenceEvent>("preference-event"),
	windowLoadedEvent: makeEvent<WindowLoadedEvent>("window-loaded-event"),
};

/* Types */
export type Action =
/**
 *  A device has attached. The device reference belongs to the
 *  client. It must be explicitly released, or else it will leak.
 */
"Attached" |
/**
 *  A device has detached. The device object delivered will be
 *  the same as the one delivered in the Attached notification. This
 *  device reference does not need to be released.
 */
"Detached" |
/**
 *  This notification is delivered in response to
 *
 *    1. A call to am::DeviceNotificationUnsubscribe().
 *    2. An error occurred on one of the underlying notification systems
 *       (i.e. usbmuxd or mDNSResponder crashed or stopped responding).
 *       Unsubcribing and resubscribing may recover the notification system.
 */
"NotificationStopped" | "Paired";

export type BatteryHealthSnapshot = {
	day: string,
	timestamp: number,
	maxCapacity: number,
	designCapacity: number,
	cycleCount: number,
};

export type ChargingHistory = {
	id: number,
	fromLevel: number,
	endLevel: number,
	chargingTime: number,
	timestamp: number,
	name: string,
	udid: string,
	isRemote: number,
	adapterName: string,
};

export type ChargingHistoryDetail = {
	avg: NormalizedData,
	peak: NormalizedData,
	curve: NormalizedResource[],
	raw: string[],
};

export type DeviceEvent = {
	udid: string,
	name: string,
	interface: InterfaceType,
	action: Action,
};

export type DevicePowerTickEvent = {
	udid: string,
	data: NormalizedResource,
};

export type Duration = {
	secs: number,
	nanos: number,
};

export type HistoryRecordedEvent = null;

export type InterfaceType = "Unknown" | "USB" | "WiFi";

export type NormalizedData = {
	systemIn: number | null,
	systemLoad: number | null,
	batteryPower: number | null,
	adapterPower: number | null,
	efficiencyLoss: number | null,
	/**  0 if not available */
	brightnessPower: number | null,
	/**  0 if not available */
	heatpipePower: number | null,
	batteryLevel: number,
	absoluteBatteryLevel: number | null,
	temperature: number | null,
	adapterWatts: number | null,
	adapterVoltage: number | null,
	adapterAmperage: number | null,
};

export type NormalizedResource = {
	isLocal: boolean,
	powerEstimated?: boolean,
	isCharging: boolean,
	/**
	 *  True when the pack is full. Distinct from `is_charging`, which is
	 *  false while resting on the adapter at 100%.
	 */
	fullyCharged?: boolean,
	/**  True when an adapter is plugged in, whether or not it is charging. */
	externalConnected?: boolean,
	timeRemain: Duration,
	timeRemainKnown?: boolean,
	lastUpdate: number,
	adapterName: string | null,
	/**  Adapter model description, e.g. "pd charger". */
	adapterDescription?: string | null,
	/**  Negotiated USB-C PD power tier. */
	adapterPowerTier?: number,
	adapterIsWireless?: boolean,
	cycleCount: number,
	currentCapacity: number,
	maxCapacity: number,
	designCapacity?: number,
} & NormalizedData;

export type PowerTickEvent = {
	data: NormalizedResource,
};

export type PowerUpdatedEvent = string;

export type PreferenceEvent = ({ theme: Theme }) & { animationsEnabled?: never; language?: never; statusBarItem?: never; statusBarShowCharging?: never; updateInterval?: never } | ({ animationsEnabled: boolean }) & { language?: never; statusBarItem?: never; statusBarShowCharging?: never; theme?: never; updateInterval?: never } | ({ updateInterval: number }) & { animationsEnabled?: never; language?: never; statusBarItem?: never; statusBarShowCharging?: never; theme?: never } | ({ language: string }) & { animationsEnabled?: never; statusBarItem?: never; statusBarShowCharging?: never; theme?: never; updateInterval?: never } | ({ statusBarItem: StatusBarItem }) & { animationsEnabled?: never; language?: never; statusBarShowCharging?: never; theme?: never; updateInterval?: never } | ({ statusBarShowCharging: boolean }) & { animationsEnabled?: never; language?: never; statusBarItem?: never; theme?: never; updateInterval?: never };

export type ProcessEnergy = {
	pid: number,
	name: string,
	/**
	 *  Relative power score reported by `top`.
	 *  Comparable between processes, not a wattage.
	 */
	impact: number | null,
};

/**
 *  Which power metric to show in the status bar.
 *
 *  Implements a forgiving `Deserialize`: unknown or non-string values (e.g. a
 *  stale `"none"` or `null` persisted by older builds) fall back to `System`
 *  instead of panicking inside tauri-specta and killing the power-tick task.
 */
export type StatusBarItem = "system" | "screen" | "heatpipe";

export type Theme = "light" | "dark" | "system";

export type WindowLoadedEvent = null;

/* Tauri Specta runtime */
async function typedError<T, E>(result: Promise<T>): Promise<{ status: "ok"; data: T } | { status: "error"; error: E }> {
    try {
        return { status: "ok", data: await result };
    } catch (e) {
        if (e instanceof Error) throw e;
        return { status: "error", error: e as any };
    }
}

type EventEmit<T> = [T] extends [null] ? () => Promise<void> : (payload: T) => Promise<void>;

function makeEvent<T>(name: string, serialize?: (payload: T) => unknown, deserialize?: (payload: any) => T) {
    const mapEvent = (cb: __TAURI_EVENT.EventCallback<T>) => (event: __TAURI_EVENT.Event<any>) => cb({ ...event, payload: deserialize ? deserialize(event.payload) : event.payload });
    const mapPayload = (payload: T) => serialize ? serialize(payload) : payload;

    const base = {
        listen: (cb: __TAURI_EVENT.EventCallback<T>) => __TAURI_EVENT.listen(name, mapEvent(cb)),
        once: (cb: __TAURI_EVENT.EventCallback<T>) => __TAURI_EVENT.once(name, mapEvent(cb)),
        emit: ((payload: T) => __TAURI_EVENT.emit(name, mapPayload(payload)) as unknown) as EventEmit<T>
    };

    const fn = (target: import("@tauri-apps/api/webview").Webview | import("@tauri-apps/api/window").Window) => ({
        listen: (cb: __TAURI_EVENT.EventCallback<T>) => target.listen(name, mapEvent(cb)),
        once: (cb: __TAURI_EVENT.EventCallback<T>) => target.once(name, mapEvent(cb)),
        emit: ((payload: T) => target.emit(name, mapPayload(payload)) as unknown) as EventEmit<T>
    });

    return Object.assign(fn, base);
}
