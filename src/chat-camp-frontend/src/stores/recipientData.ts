import { defineStore } from 'pinia';
import { useLoginData, type UserData } from './loginData';
import { computed, ref } from 'vue';
import { Principal } from '@dfinity/principal';

// export const useRecipientData = defineStore('recipientData', {
// 	state: (): State => {
// 		return {
// 			recipient: '',
// 			userData: undefined,
// 		};
// 	},
// 	actions: {
// 		userData() {
// 			if (this.userData != undefined) return this.userData;
// 			const loginData = useLoginData();
// 			loginData.backend.get_user_data();
// 		},
// 	},
// });

export const useRecipientData = defineStore('recipientData', () => {
	const loginData = useLoginData();
	const recipient = ref('');
	const userDataPriv = ref<UserData | undefined>(undefined);

	const userData = computed(() => {
		if (userDataPriv.value != undefined) return userDataPriv.value;
		loginData.getUserData(Principal.fromText(recipient.value)).then((v) => {
			userDataPriv.value = v;
		});
		return { name: 'Loading' } as UserData;
	});

	return { userData, userDataPriv, recipient };
});
