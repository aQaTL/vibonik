<template>
	<div id="container">
		<div id="top-bar">
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
	</div>
</template>
<script>
	import {API} from "../main";

	export default {
		name: "Login",

		data() {
			return {
				FB: null,
				fbCredentials: {
					accessToken: "",
					expiresIn: "",
					signedRequest: "",
					userID: "",
				},
			}
		},

		mounted() {
			this.loadFacebook();
		},

		methods: {
			loadFacebook: async function () {
				let js, fjs = document.getElementsByTagName("script")[0];
				if (document.getElementById("facebook-jssdk") || window.fbAsyncInit !== undefined) {
					FB.XFBML.parse();
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
					window.FB = FB;
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
					body: JSON.stringify({
							type: "facebookAuth",
							...this.fbCredentials,
					}),
				});
				switch (resp.status) {
					case 200: {
						let respData = await resp.json();
						switch (respData.authStatus) {
							case "success":
								this.$store.commit('setUser', respData);
								console.log("Login successful: ", respData);
								this.$emit("login");
								break;
							case "newUser":
								this.$store.commit('setUser', respData);
								console.log("New user: ", respData);
								this.$emit("login");
								await this.$router.push("/profile");
								break;
							case "fail":
								console.log("Failed to auth");
								break;
							default:
								console.log(`unknown authStatus: `, respData);
						}
						break;
					}
					case 201:
					case 500:
				}
			},

			facebookLoaded: async function () {
				this.facebookLogIn();
			},
		},
	}
</script>
<style scoped>
	#container {
		background-color: rgba(15, 15, 15, 0.35);
		display: grid;
		justify-items: center;
		padding: 2em 0.2em 0em 0.2em;
	}
</style>
