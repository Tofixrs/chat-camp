<script setup lang="ts">
import Header from "./components/Header.vue"
import Messages from "./components/Messages.vue";
import UserList from "./components/UserList.vue";
import RegisterPage from "./components/RegisterPage.vue";
import { useLoginData } from "./stores/loginData";
import { useRecipientData } from "./stores/recipientData";

const loginData = useLoginData();
const recipientData = useRecipientData();
loginData.restoreLogin();

</script>


<style>
main {
    flex-grow: 1;
    display: flex;
    max-height: 100%;
}

.split {
    display: grid;
    flex-grow: 1;
    grid-template-columns: 0.1fr 1fr;
}
</style>

<template>
    <Header />
    <main v-if="loginData.identity != undefined && loginData.userData != undefined">
        <div class="split" v-if="loginData.isLoggedIn()">
            <UserList />
            <Messages v-if="recipientData.recipient != ''" />
        </div>
    </main>
    <RegisterPage v-if="loginData.identity != undefined && loginData.userData == undefined" />
</template>
