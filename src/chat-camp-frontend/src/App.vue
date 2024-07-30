<script lang="ts">
import { createActor, canisterId } from '../../declarations/chat-camp-backend/index';
import { AuthClient } from '@dfinity/auth-client';
import { type Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { watch, watchEffect } from 'vue';

export default {
  data() {
    return {
      newMsg: "",
      chat: [] as string[],
      identity: undefined as Identity | undefined,
      backend: createActor(canisterId),
      recipient: "",
    }
  },
  methods: {
    async addMsg() {
      const text = this.newMsg;
      this.newMsg = ""
      await this.backend.add_chat_msg(text, this.getPrincipal())
      await this.getChat();
    },
    async getChat() {
      if (!this.isLoggedIn()) {
        throw new Error("Anon")
      };
      if (!this.isTalkingToSomeone()) {
        throw new Error("Didnt specify who ya talking  to");
      }

      const [chat] = await this.backend.get_chat(this.getPrincipal())
      this.chat = chat ? chat : [];
    },
    async login() {
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://a3shf-5eaaa-aaaaa-qaafa-cai.localhost:4943/",
        onSuccess: async () => {
          this.identity = authClient.getIdentity();
          this.backend = createActor(canisterId, { agentOptions: { identity: this.identity } })
        },
      });
    },
    isLoggedIn() {
      return !!this.identity && !this.identity.getPrincipal().isAnonymous()
    },
    isTalkingToSomeone() {
      return this.recipient != ""
    },
    getPrincipal() {
      const principal = Principal.fromText(this.recipient.trim());
      if (!principal || principal.isAnonymous()) {
        throw "Anon"
      }

      return principal
    },
  },
}
</script>

<style>
nav {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  padding: 5px 10px;
  background-color: darkgray;
}

.nav-start,
.nav-center,
.nav-end {
  display: flex;
  align-items: center;
}

nav .nav-end {
  display: flex;
  justify-content: flex-end;
}
</style>

<template>
  <header>
    <nav>
      <div class="nav-start">
        <i class="fa-regular fa-user"></i>
        {{ identity?.getPrincipal() ? identity.getPrincipal() : "Not logged in" }}
      </div>
      <div class="nav-center"></div>
      <div class="nav-end">
        <button @click="login">Login</button>
      </div>
    </nav>
  </header>
  <main class="msgs">
    <div class="add-note-form">
      <div>
        <label for="recipient">Recipient</label>
        <input type="text" id="recipient" v-model="recipient">
        <button @click="getChat">Refresh</button>
      </div>
      <div v-for="msg in chat" class="note">
        <p>{{ msg }}</p>
      </div>
      <div>
        <textarea v-model="newMsg"></textarea>
      </div>
      <div>
        <button @click="addMsg" :disabled="!isLoggedIn() || !isTalkingToSomeone()">Dodaj notke</button>
      </div>
    </div>
  </main>
</template>
