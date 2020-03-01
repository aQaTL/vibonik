import Vue from 'vue'
import App from './App.vue'
import VueRouter from 'vue-router'
import Profile from "./components/Profile";
import Ticket from "./components/Ticket";
import Home from "./components/Home";

const Users = () => import(/* webpackChunkName: "adminPanel" */ "./components/Users.vue");

Vue.config.productionTip = false;

export let API = Vue.config.devtools ? "http://localhost:8081/api/" : "/api/";

Vue.use(VueRouter);

const router = new VueRouter({
	mode: "history",
	routes: [
		{path: "/", component: Home},
		{path: "/profile", component: Profile},
		{path: "/ticket", component: Ticket},
		{path: "/users", component: Users},
	]
});

new Vue({
	router,
	render: h => h(App),
}).$mount('#app');
