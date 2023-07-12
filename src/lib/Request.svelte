<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let url = "";

  async function make_request(){
    let request = {
      url: "https://jsonplaceholder.typicode.com/posts",
      body: "something BODY",
      params:  "value",
      method: "GET",
    }
    console.log(url);
    await invoke("make_request", {request: request})
  }
</script>

<div>
  <form on:submit|preventDefault={make_request}>
    <div class="row">
      <select name="http-method" >
        <option value="GET">GET</option>
        <option value="POST">POST</option>
        <option value="PUT">PUT</option>
        <option value="PATCH">PATCH</option>
        <option value="DELETE">DELETE</option>
        <option value="HEAD">HEAD</option>
        <option value="OPTIONS">OPTIONS</option>
      </select>
      <input id="request-input" placeholder="Enter URL" />
      <button type="submit">GO</button>
    </div>
    <div class="row">
      <div class="tabs">
        <div>Headers</div>
        <div>Body</div>
        <div>Auth</div>
        <div>Cookies</div>
      </div>
    </div>
  </form>
</div>
