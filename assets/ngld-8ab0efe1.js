import"./modulepreload-polyfill-ec808ebb.js";(function(){let s=/[\?&]OVERLAY_WS=([^&]+)/.exec(location.href),i=null,l=[],y=0,d={},r={},o=null,f=!1;if(s){let n=function(){i=new WebSocket(s[1]),i.addEventListener("error",e=>{console.error(e)}),i.addEventListener("open",()=>{console.log("Connected!");let e=l;l=null;for(let t of e)o(t)}),i.addEventListener("message",e=>{try{e=JSON.parse(e.data)}catch{console.error("Invalid message received: ",e);return}e.rseq!==void 0&&d[e.rseq]?(d[e.rseq](e),delete d[e.rseq]):c(e)}),i.addEventListener("close",()=>{l=[],console.log("Trying to reconnect..."),setTimeout(()=>{n()},300)})};o=e=>{l?l.push(e):i.send(JSON.stringify(e))},n()}else{let n=function(){if(!window.OverlayPluginApi||!window.OverlayPluginApi.ready){setTimeout(n,300);return}let e=l;l=null,window.__OverlayCallback=c;for(let[t,a]of e)o(t,a)};o=(e,t)=>{l?l.push([e,t]):OverlayPluginApi.callHandler(JSON.stringify(e),t)},n()}function c(n){if(r[n.type])for(let e of r[n.type])e(n)}window.dispatchOverlayEvent=c,window.addOverlayListener=(n,e)=>{f&&r[n]&&console.warn(`A new listener for ${n} has been registered after event transmission has already begun.
Some events might have been missed and no cached values will be transmitted.
Please register your listeners before calling startOverlayEvents().`),r[n]||(r[n]=[]),r[n].push(e)},window.removeOverlayListener=(n,e)=>{if(r[n]){let t=r[n],a=t.indexOf(e);a>-1&&t.splice(a,1)}},window.callOverlayHandler=n=>{let e;return i?(n.rseq=y++,e=new Promise(t=>{d[n.rseq]=t}),o(n)):e=new Promise(t=>{o(n,a=>{t(a==null?null:JSON.parse(a))})}),e},window.startOverlayEvents=()=>{f=!1,o({call:"subscribe",events:Object.keys(r)})}})();let u=document.getElementById("token");addOverlayListener("onMJICraftworkData",s=>{console.log(`MJI Craftwork DATA: ${JSON.stringify(s)}`),v(s.data)});startOverlayEvents();let w=document.getElementById("close");w!=null&&(w.onclick=()=>{v("")});function v(s){u!=null&&(u.innerText=s)}
