<script setup lang="ts">

import LeftInfoList from "./LeftInfoList.vue";
import {ref} from "vue";

const ObjUrl = ref<string>("");

function setImage (blob: string) {
  ObjUrl.value = blob
}

</script>

<template>

    <v-row  no-gutters>
        <v-col cols ="4" class="bg-amber-lighten-5">
          <left-info-list :selectFile= setImage></left-info-list>
        </v-col>
        <v-col class="bg-amber-accent-1 pa-2">
            <v-img alt="Image" class="fill-height"
            eager=""
                   :src="ObjUrl"
            />
        </v-col>

    </v-row>

</template>

<style scoped>

</style>