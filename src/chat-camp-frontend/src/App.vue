<script lang="ts">
import { createActor, canisterId } from '../../declarations/chat-camp-backend/index';
import { AuthClient } from '@dfinity/auth-client';
import { type Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { watch, watchEffect } from 'vue';
import type { Message, UserData } from '../../declarations/chat-camp-backend/chat-camp-backend.did';

export default {
  data() {
    return {
      newMsg: "",
      chat: [] as Message[],
      identity: undefined as Identity | undefined,
      backend: createActor(canisterId),
      recipient: "",
      recipientUserData: undefined as UserData | undefined,
      userData: undefined as UserData | undefined,
      usernameInput: "",
      users: [] as [Principal, UserData][],
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

          await this.getUserData()
          await this.backend.get_users()
        },
      });
    },
    async logout() {
      const authClient = await AuthClient.create();
      authClient.logout();
      this.identity = undefined;
      this.backend = createActor(canisterId);
      this.chat = [];
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
    getAvatarUrl(user: UserData | undefined) {
      if (!user) return;
      const [url] = user.avatar_url;
      return url;
    },
    async register() {
      await this.backend.register(this.usernameInput.trim())
      this.userData = { name: this.usernameInput.trim(), avatar_url: [] }
      await this.getUserData();
    },
    async getUserData() {
      if (!this.isLoggedIn()) {
        throw "Anon"
      }
      const [userData] = await this.backend.get_user_data(this.identity!.getPrincipal())
      this.userData = userData;
    },
    getAuthorName(principal: Principal) {
      if (principal == this.identity?.getPrincipal()) {
        return this.userData?.name
      }
      return this.recipientUserData?.name
    }
  },
  async mounted() {
    this.users = await this.backend.get_users();
  },
  watch: {
    async recipient(_: string, newRecipient: string) {
      if (newRecipient == "") return;
      const [data] = await this.backend.get_user_data(Principal.fromText(newRecipient));
      this.recipientUserData = data;
    }
  }
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

main {
  display: grid;
  grid-template-columns: 0.1fr 1fr;
}

.userList>select {
  width: 100%;
}
</style>

<template>
  <header>
    <nav>
      <div class="nav-start">
        <i class="fa-regular fa-user" v-if="getAvatarUrl(userData) == undefined"></i>
        <img alt="" width="15" height="15" v-if="getAvatarUrl(userData) != undefined" :src="getAvatarUrl">
        <p v-if="isLoggedIn()">
          {{ userData?.name == undefined ? identity?.getPrincipal() : userData?.name }}
        </p>
        <p v-if="!isLoggedIn()"> Not logged in </p>
      </div>
      <div class="nav-center"></div>
      <div class="nav-end">
        <button @click="login" v-if="!isLoggedIn()">Login</button>
        <button @click="logout" v-if="isLoggedIn()">Log out</button>
      </div>
    </nav>
  </header>
  <main class="main" v-if="isLoggedIn() && userData">
    <div class="userList">
      <select v-model="recipient">
        <option value="" disabled>Please select one</option>
        <option v-for="[principal, user] in users" :value="principal.toText()">
          <img :src="getAvatarUrl(user)" alt="" v-if="user.avatar_url.length != 0" width="15" height="15"> <span>{{ user.name }}</span>
        </option>
      </select>
    </div>
    <div class="add-note-form">
      <div>
        <button @click="getChat">Refresh</button>
      </div>
      <div class="msgs">
        <div class="msg" v-for="msg in chat">
          <p>{{ getAuthorName(msg.author as Principal /*ts being weird*/) }} {{ new Date(Number(msg.timestamp) / 1000000).toString() }}</p>
          <p>{{ msg.content }}</p>
        </div>
      </div>
      <div>
        <textarea v-model="newMsg"></textarea>
      </div>
      <div>
        <button @click="addMsg" :disabled="!isLoggedIn() || !isTalkingToSomeone()">Dodaj notke</button>
      </div>
    </div>
  </main>
  <main v-if="isLoggedIn() && !userData">
    <label for="Nickname">Nickname</label>
    <input type="text" id="Nickname" v-model="usernameInput">
    <div>
      <button @click="register">Register</button>
    </div>
  </main>
</template>
