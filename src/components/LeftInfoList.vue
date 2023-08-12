<template>
  <v-container class="h-screen d-flex flex-column ma-0 pa-2">
      <top-btn-bar :buttons = "buttons" ></top-btn-bar>
      <v-card loading class="pa-2 h-100">
        <v-card-item class="h-100 overflow-y-auto overflow-x-hidden elevation-2"
        style="align-content: start;"
        >
        <expandable-list
        :items="listInfo"
        ></expandable-list>
        </v-card-item>
      </v-card>
  </v-container>
</template>

<script lang="ts" setup>

import {dialog, fs, tauri,} from "@tauri-apps/api";
import {computed, ref} from "vue";
import { ExpandItem } from "../dto/expandable";
import {blobToBase64, BtnDefine, PreviewImage} from "../tauri_pack/pack.ts";
import TopBtnBar from "./TopBtnBar.vue";
import ExpandableList from "./ExpandableList.vue";


const progress = ref(75.0);
const listInfo:ExpandItem[] = [
  {
    name: "A",
    icon:"mdi-close",
    onClick:()=>{},
    childen:[
      {name:"B",onClick:()=>{},icon:"mdi-image"},
      {name:"B",onClick:()=>{},icon:"mdi-cof"},
    ],
  },
  {
    name: "b",
    icon:"mdi-close",
    onClick:()=>{},
    childen:[
      {name:"B",onClick:()=>{},icon:"mdi-image"},
      {name:"B",onClick:()=>{}},
    ],
  },
  {
    name: "A",
    icon:"mdi-close",
    onClick:()=>{},
    childen:[
      {name:"B",onClick:()=>{},icon:"mdi-image"},
      {name:"B",onClick:()=>{}},
    ],
  },
  {
    name: "A",
    icon:"mdi-close",
    onClick:()=>{},
    childen:[
      {name:"B",onClick:()=>{},icon:"mdi-image"},
      {name:"B",onClick:()=>{}},
    ],
  },
  {
    name: "A",
    icon:"mdi-close",
    onClick:()=>{},
    childen:[
      {name:"B",onClick:()=>{},icon:"mdi-image"},
      {name:"B",onClick:()=>{}},
    ],
  },
  {
    name: "A",
    icon:"mdi-close",
    onClick:()=>{},
    childen:[
      {name:"B",onClick:()=>{},icon:"mdi-image"},
      {name:"B",onClick:()=>{}},
    ],
  },
  {
    name: "A",
    icon:"mdi-close",
    onClick:()=>{},
    childen:[
      {name:"B",onClick:()=>{},icon:"mdi-image"},
      {name:"B",onClick:()=>{}},
    ],
  },
  // {
  //   id: 1,
  //   text: "B",
  //   originFile: "ABCGG",
  // },
  // {
  //   id: 2,
  //   text: "C",
  //   originFile: "ABCDD",
  // }, {
  //   id: 3,
  //   text: "C",
  //   originFile: "ABCDD",
  // },
  // {
  //   id: 4,
  //   text: "C",
  //   originFile: "ABCDD",
  // },
  // {
  //   id: 5,
  //   text: "C",
  //   originFile: "ABCDD",
  // }, {
  //   id: 4,
  //   text: "C",
  //   originFile: "ABCDD",
  // },
  // {
  //   id: 5,
  //   text: "C",
  //   originFile: "ABCDD",
  // },
  // {
  //   id: 4,
  //   text: "C",
  //   originFile: "ABCDD",
  // },
  // {
  //   id: 6,
  //   text: "C",
  //   originFile: "ABCDD",
  // }
]


const onClick = async () => {
  await dialog.message("Now Import Unity Bound file", {okLabel: "确定", type: "info", title: "Message",})
}

const onSelectFile = async () => {
  let path = await dialog.open({
        title: "Select File",
        multiple: false,
        directory: false,
        recursive: false,
        filters: [{name: "Image", extensions: ["png", "jpg", "jpeg"]}]
      }
  )
  if (path == null || (typeof path == "object" && path.length == 0)) {
    return
  }
  let filename = typeof path == "string" ? path : path[0]
  console.log(filename)
  let stream = await fs.readBinaryFile(filename,)
  console.log(stream.length)
  let blob = new Blob([stream],)
  console.log(blob)

  // property.selectFile(URL.createObjectURL(blob))
  await PreviewImage({name: filename, obj_url: await blobToBase64(blob), description: "Image from File"})
}

const onLoadUnity = async () => {
  let path = await dialog.open({
        title: "Select File",
        multiple: false,
        directory: false,
        recursive: false,
      }
  )
  if (path == null || (typeof path == "object" && path.length == 0)) {
    return
  }
  let filename = typeof path == "string" ? path : path[0]
  console.log(filename)

  let ret = await tauri.invoke<{
    img: string, width: number, height: number, name: string
  }>("extractor_unity_img", {filename: filename})
      .catch((err: { msg: string }) => {
        alert(`error unpack ${err.msg}`)
      })
  console.log(ret)
  if (ret != null) {
    await PreviewImage({
      name: filename,
      obj_url: ret.img,
      description: `Image From Unity File [${ret.name}]`,
      width: ret.width,
      height: ret.height
    })
  }
}

const buttons:BtnDefine[] = [
  {
    icon:"mdi-file-image-outline",
    onClick:onSelectFile,
    tooltip:"Open an Image",
  },
  {
    icon:"mdi-unity",
    onClick:onLoadUnity,
    tooltip:"Open an Image",
  },{
    icon:"mdi-cog-outline",
    onClick:onClick,
    tooltip:"Open an Image",
  },{
    icon:"mdi-file-search-outline",
    onClick:()=>{},
    tooltip:"Search",
  },{
    icon:"mdi-close-circle-outline",
    onClick:()=>{},
    tooltip:"Quit",
  }
]

</script>

<style scoped>
.scroll {
  overflow: scroll;
}
</style>