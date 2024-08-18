<script setup lang="ts">
import { ref, type Ref } from 'vue';
import type { UserData } from '../../../declarations/chat-camp-backend/chat-camp-backend.did';
import { useLoginData } from '../stores/loginData';
import { useRecipientData } from '../stores/recipientData';
import { Principal } from '@dfinity/principal';
const loginData = useLoginData();
const recipientData = useRecipientData();
const users = ref<[Principal, UserData][]>([])
loginData.backend.get_users().then((v) => {
    users.value = v;
});

</script>

<style>
.userList {
    max-height: 100%;
    background-color: gray;
    padding-inline: 0.5em;
}
</style>

<template>
    <div class="userList">
        <select v-model="recipientData.recipient">
            <option disabled>Please select one</option>
            <option v-for="[principal, user] in users" :value="principal.toText()">
                <img :src="user.avatar_url[0]" v-if="user.avatar_url.length != 0" width="15" height="15"> <span>{{ user.name }}</span>
            </option>
        </select>
    </div>
</template>