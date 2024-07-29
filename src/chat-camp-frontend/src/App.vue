<script lang="ts">
import { ref } from 'vue';
import { createActor, canisterId } from '../../declarations/chat-camp-backend/index';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent, type Identity } from '@dfinity/agent';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[],
      identity: undefined as Identity | undefined,
      backend: createActor(canisterId)
    }
  },
  methods: {
    async addNote() {
      await this.backend.add_note(this.newNote);
      this.newNote = ""
      await this.getNotes();
    },
    async getNotes() {
    },
    async login() {
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://a3shf-5eaaa-aaaaa-qaafa-cai.localhost:4943/",
      });
      this.identity = authClient.getIdentity();
      const agent = await HttpAgent.create({ identity: this.identity })
      this.backend = createActor(canisterId, { agent })
    }
  },
  mounted() {
    this.getNotes()
  },
}

</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <button @click="login">Login</button>
    {{ identity?.getPrincipal() }}
    <div v-for="note in notes">
      <p>{{ note }}</p>
    </div>
    <div>
      <textarea v-model="newNote"></textarea>
      <button @click="addNote">Dodaj notke</button>
    </div>
  </main>
</template>
