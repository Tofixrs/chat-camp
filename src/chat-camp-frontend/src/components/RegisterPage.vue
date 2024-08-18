<script lang="ts" setup>
import { ref } from 'vue';
import { useLoginData } from '../stores/loginData';

const username = ref("");
const loginData = useLoginData();

async function register() {
    await loginData.backend.register(username.value)
    loginData.userData = await loginData.getUserData(loginData.identity!.getPrincipal());
}
</script>

<template>
    <main>
        <label for="name">Nickname</label>
        <input type="text" name="name" id="name" v-model="username">
        <div>
            <button @click="register">Register</button>
        </div>
    </main>
</template>