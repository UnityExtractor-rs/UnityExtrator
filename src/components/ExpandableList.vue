<template>
  <v-list>
    <v-list-group v-for="(item,i) in property.items"
    :value="i" :key="i"
    >
        <template v-slot:activator="{props}">
            <v-list-item
                v-bind="props"
                :prepend-icon="item.icon"
                :title="item.name"
            >
            </v-list-item>
        </template>
        <v-list-item v-for="(child,i) in item.childen"
            :key="i"
            :prepend-icon="child.icon"
            :title="child.name"
            @click="child.onClick"
        >

        </v-list-item>
    </v-list-group>
    <v-list-item type = "divider"></v-list-item>
  </v-list>
</template>

<script setup lang="ts">
import {ExpandableItem} from "./ExpandableList.vue"
import { defineProps } from 'vue';
import {ExpandItem } from "../dto/expandable"
const property = defineProps<{
    items:ExpandItem[]
}>()

</script>

<style>

</style>