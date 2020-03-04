<template>
	<div class="hello">
		<h1>asdf</h1>
		<form v-on:submit.prevent="">
			<input type="text" v-model="echoInput" v-on:keypress.enter="echo">
		</form>
	</div>
</template>

<script>
	import {API} from "../main";

	export default {
		name: 'HelloWorld',
		props: {
			msg: String
		},
		data() {
			return {
				echoInput: "",
			}
		},
		methods: {
			echo: async function () {
				let url = new URL(API + "echo");
				url.searchParams.append("msg", this.echoInput);
				let res = await fetch(url, {
					method: "GET",
				});
				console.log(await res.text());
			}
		},
	}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
	h3 {
		margin: 40px 0 0;
	}
	ul {
		list-style-type: none;
		padding: 0;
	}
	li {
		display: inline-block;
		margin: 0 10px;
	}
	a {
		color: #42B983;
	}
</style>
