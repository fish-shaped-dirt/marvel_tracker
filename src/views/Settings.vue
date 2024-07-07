<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
  data() {
    return {
      client_key: "",
      secret_key: "",
    };
  },
  methods: {
    async save_api2() {
      invoke("save_api", {
        key1: this.client_key,
        key2: this.secret_key,
      });
    },
  },
};
</script>

<template>
  <h1>Settings</h1>

  <form class="input_field" @submit.prevent="save_api2">
    <input placeholder="client key" autocomplete="off" v-model="client_key" />
    <input placeholder="secret key" autocomplete="off" v-model="secret_key" />
    <button type="submit">
      <span class="material-symbols-rounded">save</span>
    </button>
  </form>
</template>
