import { Event, UnlistenFn, emit } from "@tauri-apps/api/event"
import { Ref } from "vue"
import { appWindow } from "@tauri-apps/api/window"


export interface ImagePreview {
    imageUrl: string,
    name: string,
    description?: string,
    fromat: string,
    width: number,
    height: number
}

export interface TextPreview {
    payload: string,
    name: string,
    description?: string,
}

export interface PreviewUnLisen {
    image: UnlistenFn,
    text: UnlistenFn
}

export enum PreviewMode {
    Image,
    Text
}

export async function lisenPreview(state: Ref<null | ImagePreview | TextPreview>, mode: Ref<PreviewMode | null>): Promise<PreviewUnLisen> {
    let image = await appWindow.listen<ImagePreview>("preview_image", ({ _, payload }: Event<ImagePreview>) => {
        state.value = payload
        mode.value = PreviewMode.Image
        console.log(payload);

        emit("loading", false)
    })

    let text = await appWindow.listen<TextPreview>("preview_text", (event: Event<TextPreview>) => {
        state.value = event.payload;
        mode.value = PreviewMode.Text
        console.log(event.payload);

        emit("loading", false)
    })

    return { image, text }
}

export async function unListenAll(unlisten: PreviewUnLisen) {
    unlisten.image()
    unlisten.text()
}