import Vue from 'vue'
import App from './App.vue'

Vue.config.productionTip = false;

export let API = Vue.config.devtools ? "http://localhost:8081/api/" : "/api/";

console.log(Vue.config);

new Vue({
	render: h => h(App),
}).$mount('#app');
