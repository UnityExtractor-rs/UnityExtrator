<template>
  <v-container class="h-screen">
    <v-row align-content="start" class="h-auto">
      <v-col>
        <v-btn class="text-amber-darken-3" @click="onClick">
          <v-icon icon="mdi-plus"/>
        </v-btn>
      </v-col>
      <v-col>
        <v-btn class="text-amber-darken-3" @click="onSelectFile">
          <v-icon icon="mdi-image-plus"/>
        </v-btn>
      </v-col>
      <v-col>
        <v-btn class="text-amber-darken-3" @click="onLoadUnity">
          <v-icon icon="mdi-cog"/>
        </v-btn>
      </v-col>
    </v-row>
    <v-row class="pa-2" style="height: 84%">

      <v-card class=" h-100 overflow-y-auto overflow-x-hidden pa-2"
      >

        <v-expansion-panels
            multiple=""
            variant="accordion">
          <v-expansion-panel
              v-for="item in listInfo"
              :key="item.id"
              :title="item.text"
          >
            <v-expansion-panel-text class="pa-0 ma-0">
              <v-list class="align-start">
                <v-list-item>
                  <v-icon icon="mdi-file-document"></v-icon>
                  {{ item.originFile }}
                </v-list-item>
                <v-list-item>
                  <v-icon icon="mdi-cog"></v-icon>
                  {{ item.originFile }}
                </v-list-item>
              </v-list>
            </v-expansion-panel-text>

          </v-expansion-panel>
        </v-expansion-panels>
      </v-card>
    </v-row>
    <v-row align-content="space-evenly" class="pa-2">
      <v-progress-linear color="primary" height="10" indeterminate="" rounded/>

    </v-row>
  </v-container>
</template>

<script lang="ts" setup>

import {dialog, fs, tauri,} from "@tauri-apps/api";
import {computed, ref} from "vue";

const property = defineProps<{
  selectFile: (String ) => void
}>()

const progress = ref(75.0);
const listInfo = ref([
  {
    id: 0,
    text: "A",
    originFile: "ABC",
  },
  {
    id: 1,
    text: "B",
    originFile: "ABCGG",
  },
  {
    id: 2,
    text: "C",
    originFile: "ABCDD",
  }, {
    id: 3,
    text: "C",
    originFile: "ABCDD",
  },
  {
    id: 4,
    text: "C",
    originFile: "ABCDD",
  },
  {
    id: 5,
    text: "C",
    originFile: "ABCDD",
  }, {
    id: 4,
    text: "C",
    originFile: "ABCDD",
  },
  {
    id: 5,
    text: "C",
    originFile: "ABCDD",
  },
  {
    id: 4,
    text: "C",
    originFile: "ABCDD",
  },
  {
    id: 6,
    text: "C",
    originFile: "ABCDD",
  }
])


const onClick = async () => {
  await dialog.message("Now Import Unity Bound file", {okLabel: "确定", type: "info", title: "Message",})
}

const onSelectFile = async () => {
  let path = await dialog.open({
        title: "Select File",
        multiple: false,
        directory: false,
        recursive: false,
        filters: [{name: "Image", extensions: ["png","jpg","jpeg"]}]
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
  console.log(property.selectFile)

  property.selectFile(URL.createObjectURL(blob))
}

const onLoadUnity=async ()=>{
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

  let ret = await tauri.invoke<null>("extractor_unity_img",{filename:filename})
      .catch((err:{msg:string})=> {
        alert(`error unpack ${err.msg}`)
      })
  console.log(ret)
  property.selectFile(ret)
}
</script>

<style scoped>
.scroll {
  overflow: scroll;
}
</style>