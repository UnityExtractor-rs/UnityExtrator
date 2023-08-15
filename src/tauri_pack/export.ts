import { dialog, invoke } from "@tauri-apps/api";
import { UnityAsset as UnityBundle, UnityObject } from "./load_unity";

export async function exportBoundle(bundle: UnityBundle) {
    //select dir to export
    let dir = await dialog.open({
        title: "Select dir to export",
        multiple: false,
        directory: true,

    })
    if (!dir) {
        return
    }
    console.log(dir);
    

    dir = typeof dir == "string" ? dir : dir[0]

    invoke("export_bundle", { dir, assetId: bundle.id })

}

export async function exportObject(asset: UnityBundle, object: UnityObject) {
    // take object extra name
    let name = await invoke<{ name: string, wildcard: string }>("export_file_type", { assetId: asset.id, objectId: object.id })

    let filename = await dialog.save({ title: "Select Save File", filters: [{ name: name.name, extensions: [name.wildcard] }], defaultPath: object.name })

    if (filename) {

        invoke("export_object", { filename, assetId: asset.id, objectId: object.id })
    }
}