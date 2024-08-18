import { type ActorSubclass, type Identity } from '@dfinity/agent';
import { defineStore } from 'pinia';
import {
	canisterId,
	createActor,
} from '../../../declarations/chat-camp-backend';
import type {
	_SERVICE,
	UserData as UserDataB,
} from '../../../declarations/chat-camp-backend/chat-camp-backend.did';
import { AuthClient } from '@dfinity/auth-client';
import { type AuthClient as AuthClientT } from '@dfinity/auth-client';
import type { Principal } from '@dfinity/principal';
export const useLoginData = defineStore('loginData', {
	state: (): State => {
		return {
			identity: undefined,
			backend: createActor(canisterId),
			userData: undefined as UserData | undefined,
		};
	},
	actions: {
		async login() {
			const authClient = await AuthClient.create();
			if (await authClient.isAuthenticated()) {
				this.identity = authClient.getIdentity();
				this.backend = createActor(canisterId, {
					agentOptions: { identity: this.identity },
				});
				this.userData = await this.getUserData(this.identity.getPrincipal());
				return;
			}

			authClient.login({
				identityProvider: 'http://a3shf-5eaaa-aaaaa-qaafa-cai.localhost:4943/',
				onSuccess: async (message) => {
					this.identity = authClient.getIdentity();
					this.backend = createActor(canisterId, {
						agentOptions: { identity: this.identity },
					});

					this.userData = await this.getUserData(this.identity.getPrincipal());
				},
			});
		},
		async restoreLogin() {
			const authClient = await AuthClient.create();

			if (!(await authClient.isAuthenticated())) return;
			this.identity = authClient.getIdentity();
			this.backend = createActor(canisterId, {
				agentOptions: { identity: this.identity },
			});
			this.userData = await this.getUserData(this.identity.getPrincipal());
		},
		async logout() {
			const authClient = await AuthClient.create();
			authClient.logout();
			this.identity = undefined;
			this.backend = createActor(canisterId);
		},
		async getUserData(prinicpal: Principal): Promise<UserData | undefined> {
			const [userData] = await this.backend.get_user_data(prinicpal);
			if (!userData) return;
			const [avatar_url] = userData.avatar_url;
			return {
				avatar_url,
				name: userData.name,
			};
		},
		isLoggedIn() {
			return (
				this.identity != undefined &&
				!this.identity.getPrincipal().isAnonymous()
			);
		},
	},
});

interface State {
	identity?: Identity;
	backend: ActorSubclass<_SERVICE>;
	userData?: UserData;
	authClient?: AuthClientT;
}

export interface UserData {
	avatar_url?: string;
	name: string;
}
