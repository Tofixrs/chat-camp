<script setup lang="ts">
import { useLoginData } from '../stores/loginData';
const loginData = useLoginData();
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
                <i class="fa-regular fa-user" v-if="loginData.userData?.avatar_url == undefined"></i>
                <img src="" alt="" width="15" height="15" v-if="loginData.userData?.avatar_url != undefined">
                <p v-if="loginData.isLoggedIn()">
                    {{ loginData.userData?.name == undefined ? loginData.identity?.getPrincipal() : loginData.userData.name }}
                </p>
                <p v-if="!loginData.isLoggedIn()">Not logged in</p>
            </div>
            <div class="nav-center"></div>
            <div class="nav-end">
                <button @click="loginData.login()" v-if="!loginData.isLoggedIn()">Login</button>
                <button @click="loginData.logout()" v-if="loginData.isLoggedIn()">Logout</button>
            </div>
        </nav>
    </header>
</template>