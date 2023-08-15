import { tauri } from "@tauri-apps/api"
import { ExpandItem } from "../dto/expandable";
import { exportBoundle, exportObject } from "./export";
import { startLoading } from "./pack";

export interface UnityAsset {
    id: string,
    name: string,
    location: string,
    assets: UnityObject[]

}

export interface UnityObject {
    id: number,
    name: string,
    icon: string,
    type: string,
    meta: string[],
}

export async function loadUnityAsset(filename: string): Promise<UnityAsset> {
    console.log("loading " + filename);

    let data = await tauri.invoke<UnityAsset>("load_unity_asset", { path: filename })
    if (!data) {
        throw new Error("cannot Load Unity Asset");
    }
    return data
}

export async function syncLoadedAsset(): Promise<UnityAsset[]> {
    let data = await tauri.invoke<UnityAsset[]>("sync_loaded_asset")

    return data
}

export async function saveObject(parent: UnityAsset, obj: UnityObject): Promise<void> {
    startLoading()
    console.log(parent, obj);
    await exportObject(parent, obj)

}

export async function preiviewObject(parent: UnityAsset, obj: UnityObject): Promise<void> {
    console.log(`load:${parent.id}-${obj.id}`);

    await tauri.invoke<void>("preview_object", { assetId: parent.id, objectId: obj.id })
}

export function unityAssetToExpandable(parent: UnityAsset): ExpandItem {
    return {
        name: parent.name,
        desription: parent.location,
        icon: "mdi-unity",
        menuItems: [
            {
                icon: "mdi-export", text: "Export Full", onClick: async () => {
                    startLoading()
                    await exportBoundle(parent).catch((err) => {
                        alert(`cannot Save Boundle: ${err}`)
                    })

                }
            }
        ],
        childen: parent.assets.map((obj: UnityObject) => {
            return {
                name: obj.name,
                icon: obj.icon,
                desription: `${obj.type}[${obj.meta.join(" ")}]`,
                onClick: async () => {
                    startLoading()
                    await preiviewObject(parent, obj).catch((err) => {
                        alert(`cannot preivew :${err}`)
                    })
                },
                menuItems: [
                    {
                        icon: "mdi-export",
                        text: "Save Object",
                        onClick: async () => {
                            await saveObject(parent, obj).catch((err) => {
                                alert(`cannot Save Object: ${err}`)
                            })

                        }
                    }
                ]
            }
        })
    }
}