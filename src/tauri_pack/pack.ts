import {tauri} from "@tauri-apps/api";

export interface ImagePayload {
    obj_url: string,
    name:string,
    description?: string,
    ty?: string,
    width?: number,
    height?: number,
}

export async function PreviewImage(payload: ImagePayload) {
    return await tauri.invoke<void>("preview_image", {
        payload
    })
}

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