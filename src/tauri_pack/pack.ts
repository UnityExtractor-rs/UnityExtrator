import { dialog, tauri, } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";


export async function blobToBase64(blob: Blob): Promise<string> {

    let promise = new Promise<string>((resolve, reject) => {
        const reader = new FileReader()
        reader.onload = (_) => {
            let result = reader.result as string;
            resolve(result)
        }
        reader.onerror = (err) => {
            reject(err)
        }

        reader.readAsDataURL(blob)
    })
    return await promise
}

export function notNullOr<T>(data: T | undefined | null, def: T): T {
    return data ? data : def
}

export interface BtnDefine {
    icon?: string,
    onClick: () => void,
    color?: string,
    tooltip?: string,
    variant?: "elevated" | "flat" | "tonal" | "outlined" | "text" | "plain",
    id?: string
}

export async function openOneFile(title?: string, filters?: DailogFilter): Promise<string | null> {
    let path = await dialog.open({
        title: title ? title : "Select a File",
        multiple: false,
        directory: false,
        recursive: false,
        filters: filters
    });
    if (path == null || (typeof path == "object" && path.length == 0)) {
        return null;
    }
    let filename = typeof path == "string" ? path : path[0];
    return filename
}


export function startLoading() {
    emit("loading", true)
}

export function loadDone() {
    emit("loading", false)
}