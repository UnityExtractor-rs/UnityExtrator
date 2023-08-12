<script setup lang="ts">

import LeftInfoList from "../components/MainPage.vue";
import {ref} from "vue";

const ObjUrl = ref<string>("");

function setImage (blob: string) {
  ObjUrl.value = blob
}

</script>

<template>

    <v-row  no-gutters>
        <v-col class="bg-amber-lighten-5">
          <left-info-list :selectFile= setImage></left-info-list>
        </v-col>
    </v-row>

</template>

<style scoped>

</style>