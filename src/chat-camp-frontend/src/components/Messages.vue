<script setup lang="ts">
import { ref, watch } from 'vue';
import { useLoginData } from '../stores/loginData';
import { useRecipientData } from '../stores/recipientData';
import type { Message } from '../../../declarations/chat-camp-backend/chat-camp-backend.did';
import { Principal } from '@dfinity/principal';
import showdown from "showdown";

const chat = ref<Message[]>([]);
const newMsg = ref("");
const loginData = useLoginData();
const recipientData = useRecipientData();
const converter = new showdown.Converter({ emoji: true, simpleLineBreaks: true, tables: true });
const messagesDiv = ref<HTMLDivElement>(null as unknown as HTMLDivElement)

async function fetchChat() {
    const [chatF] = await loginData.backend.get_chat(Principal.fromText(recipientData.recipient));
    chat.value = chatF?.reverse() || [];
}

recipientData.$subscribe(async (mut, state) => {
    await fetchChat()
})

function isMsgFromLoggedUser(msg: Message) {
    return msg.author.compareTo(loginData.identity!.getPrincipal()) == "eq";
}

function getAuthorName(msg: Message) {
    if (isMsgFromLoggedUser(msg)) {
        return loginData.userData?.name
    }
    return recipientData.userData?.name
}

async function sendMsg() {
    const text = newMsg.value;
    newMsg.value = "";
    await loginData.backend.add_chat_msg(text, Principal.fromText(recipientData.recipient));
    await fetchChat()
}

function preventEnter(evt: KeyboardEvent) {
    if (evt.key == "Enter" && !evt.shiftKey) {
        evt.preventDefault();

        sendMsg();
    }
}
</script>

<style>
.chat {
    height: 100%;
    max-height: 100%;
    display: flex;
    flex-direction: column;
}

.messages-wrapper {
    flex-grow: 1;
    height: 100%;
    width: 100%;
    position: relative;
}

.messages {
    position: absolute;
    left: 0;
    right: 0;
    top: 0;
    bottom: 0;
    overflow-y: scroll;
    padding: 1em;
    display: flex;
    flex-direction: column-reverse;
}

.chat-header {
    background-color: gray;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1em;
}

.msg-wrap {
    display: flex;
    margin: 1em 0em;
}

.msg-wrap.right {
    justify-content: flex-end;
}

.msg-wrap.left {
    justify-content: flex-start;
}

.msg-content {
    color: white;
    border-radius: 1em;
    padding: 0.5em 1em;
}

.msg-author {
    font-size: small;
}

.msg-wrap.right .msg-author {
    text-align: right;
}

.msg-wrap.right .msg-content {
    background-color: royalblue;
}

.msg-wrap.left .msg-content {
    background-color: black;
}

.input {
    display: flex;
}

.text-box {
    resize: none;
    flex-grow: 1;
    display: flex;
    align-items: center;
}
</style>

<template>
    <div class="chat">
        <div class="chat-header">
            <span>{{ recipientData.userData!.name }}</span>
            <button @click="fetchChat"><i class="fa-solid fa-arrows-rotate"></i></button>
        </div>
        <div class="messages-wrapper">
            <div class="messages" ref="messagesDiv">
                <div :class="isMsgFromLoggedUser(msg as Message) ? 'msg-wrap right' : 'msg-wrap left'" v-for="msg in chat">
                    <div class="msg">
                        <p class="msg-author">{{ getAuthorName(msg as Message) }}</p>
                        <p class="msg-content" :title="new Date(Number(msg.timestamp) / 1000000).toString()" v-html="converter.makeHtml(msg.content)"></p>
                    </div>
                </div>
            </div>
        </div>
        <div class="input">
            <textarea v-model="newMsg" class="text-box" @keydown="preventEnter"></textarea>
            <button @click="sendMsg"><i class="fa-solid fa-paper-plane"></i></button>
        </div>
    </div>
</template>