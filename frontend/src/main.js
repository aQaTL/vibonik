import Vue from 'vue'
import Vuex from 'vuex';
import App from './App.vue'
import VueRouter from 'vue-router'
import Profile from "./components/Profile";
import Ticket from "./components/Ticket";
import Home from "./components/Home";

const Users = () => import(/* webpackChunkName: "adminPanel" */ "./components/Users.vue");

Vue.config.productionTip = false;

export let API = Vue.config.devtools ? "http://localhost:8081/api/" : "/api/";

Vue.use(VueRouter);
Vue.use(Vuex);

import { library } from '@fortawesome/fontawesome-svg-core'
import { faHome, faInfo, faUser, faMusic } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

library.add(faHome, faInfo, faUser, faMusic);

Vue.component('font-awesome-icon', FontAwesomeIcon);

const router = new VueRouter({
	mode: "history",
	routes: [
		{path: "/", component: Home},
		{path: "/profile", component: Profile},
		{path: "/ticket", component: Ticket},
		{path: "/users", component: Users},
	]
});

const store = new Vuex.Store({
	state: {
		user: {
			authStatus: "",
			id: "",
			fbId: "",
			accessToken: "",
			uuid: "",
			login: "",
			passwordHash: "",
			role: "",
			name: "",
			pesel: "",
			email: "",
			birthday: "",
			gender: "",
			foodPreferences: "",
			paid: 0,
		},
	},
	mutations: {
		setUser (user) {
			console.log("[Vuex] setting user", user);
			this.state.user = user;
		},
	}
});

new Vue({
	router,
	store,
	render: h => h(App),
}).$mount('#app');
