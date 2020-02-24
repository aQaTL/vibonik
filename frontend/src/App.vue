<template>
	<div id="app">
		<HelloWorld msg="Welcome to Your Vue.js App"/>
		<div
				ref="fbLoginButton"
				class="fb-login-button"
				data-width=""
				data-size="large"
				data-button-type="login_with"
				data-auto-logout-link="true"
				data-use-continue-as="false"
				data-onlogin="document.facebookLoginCallback()"
				data-onlogout="console.log('logging OUT!')"
		>
		</div>
	</div>
</template>

<script>
	import HelloWorld from './components/HelloWorld.vue'
	import {API} from "./main";

	export default {
		name: 'App',
		components: {
			HelloWorld
		},
		mounted() {
			this.loadFacebook();
		},

		data() {
			return {
				FB: null,
				fbCredentials: {
					accessToken: "",
					expiresIn: "",
					signedRequest: "",
					userID: "",
				},
			};
		},

		methods: {
			facebookLogIn: async function () {
				console.log("facebook: ", this.FB);

				this.FB.getLoginStatus((response) => {
					console.log("calling getLoginStatus");
					if (response.status === "connected") {
						this.fbCredentials = response.authResponse;
						this.facebookLoginSuccessful();
					}
					console.log("Login response: ", response);
				});
			},

			facebookLoginSuccessful: async function () {
				this.FB.api("/me", (response) => {
					console.log("USER INFO: ", response);
				});

				let resp = await fetch(API + "auth", {
					method: "POST",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify(this.fbCredentials),
				});
				switch (resp.status) {
					case 200: {
						let authStatus = await resp.json();
						switch (authStatus) {
							default:
								console.log(`unknown authStatus: ${authStatus}`);
						}
						console.log(`AuthStatus: ${authStatus}`);
						break;
					}
					case 201:
					case 500:
				}
			},

			facebookLoaded: async function () {
				this.facebookLogIn();
			},

			loadFacebook: function () {
				let js, fjs = document.getElementsByTagName("script")[0];
				if (document.getElementById("facebook-jssdk") || window.fbAsyncInit !== undefined) {
					return;
				}
				window.fbAsyncInit = () => {
					/*eslint no-undef: "off"*/
					FB.init({
						appId: "196521161566506",
						autoLogAppEvents: true,
						xfbml: true,
						version: 'v6.0'
					});
					FB.AppEvents.logPageView();

					this.FB = FB;
					this.facebookLoaded();
				};

				document.facebookLoginCallback = () => {
					this.facebookLogIn()
				};

				js = document.createElement("script");
				js.id = "facebook-jssdk";
				js.crossOrigin = "anonymous";
				js.src = "https://connect.facebook.net/pl_PL/sdk/debug.js";
				fjs.parentNode.insertBefore(js, fjs);
			},
		}
	}
</script>

<style>
	body {
		background-color: #FEFBEC;
		color: #AE9513;
	}
	#app {
		font-family: Avenir, Helvetica, Arial, sans-serif;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		text-align: center;
		margin-top: 60px;


	}
</style>
