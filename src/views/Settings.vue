<script>
import { message } from "@tauri-apps/api/dialog";
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
      await invoke("save_api", {
        key1: this.client_key,
        key2: this.secret_key,
      });
      this.checkKeys();
    },
    async checkKeys() {
      try {
        let response = await invoke("get_api");
        document.getElementById("send").className = "ok";
        document.getElementById("send_icon").innerHTML = "check_circle";
        this.client_key = response[0];
        this.secret_key = response[1];
      } catch (e) {
        console.error(e);
      }
    },
  },
  mounted() {
    this.checkKeys();
  },
};
</script>

<template>
  <h1>Settings</h1>

  <form class="input_field" @submit.prevent="save_api2">
    <input placeholder="client key" autocomplete="off" v-model="client_key" />
    <input placeholder="secret key" autocomplete="off" v-model="secret_key" />
    <button type="submit" id="send">
      <span class="material-symbols-rounded" id="send_icon">save</span>
    </button>
  </form>
</template>
