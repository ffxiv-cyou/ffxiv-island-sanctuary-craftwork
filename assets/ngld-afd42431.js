import{a as I}from"./config-d68b2056.js";(function(){let t=/[\?&]OVERLAY_WS=([^&]+)/.exec(location.href),o=null,s=[],L=0,d={},i={},r=null,w=!1;if(t){let n=function(){o=new WebSocket(t[1]),o.addEventListener("error",e=>{console.error(e)}),o.addEventListener("open",()=>{console.log("Connected!");let e=s;s=null;for(let l of e)r(l)}),o.addEventListener("message",e=>{try{e=JSON.parse(e.data)}catch{console.error("Invalid message received: ",e);return}e.rseq!==void 0&&d[e.rseq]?(d[e.rseq](e),delete d[e.rseq]):f(e)}),o.addEventListener("close",()=>{s=[],console.log("Trying to reconnect..."),setTimeout(()=>{n()},300)})};r=e=>{s?s.push(e):o.send(JSON.stringify(e))},n()}else{let n=function(){if(!window.OverlayPluginApi||!window.OverlayPluginApi.ready){setTimeout(n,300);return}let e=s;s=null,window.__OverlayCallback=f;for(let[l,a]of e)r(l,a)};r=(e,l)=>{s?s.push([e,l]):OverlayPluginApi.callHandler(JSON.stringify(e),l)},n()}function f(n){if(i[n.type])for(let e of i[n.type])e(n)}window.dispatchOverlayEvent=f,window.addOverlayListener=(n,e)=>{w&&i[n]&&console.warn(`A new listener for ${n} has been registered after event transmission has already begun.
Some events might have been missed and no cached values will be transmitted.
Please register your listeners before calling startOverlayEvents().`),i[n]||(i[n]=[]),i[n].push(e)},window.removeOverlayListener=(n,e)=>{if(i[n]){let l=i[n],a=l.indexOf(e);a>-1&&l.splice(a,1)}},window.callOverlayHandler=n=>{let e;return o?(n.rseq=L++,e=new Promise(l=>{d[n.rseq]=l}),r(n)):e=new Promise(l=>{r(n,a=>{l(a==null?null:JSON.parse(a))})}),e},window.startOverlayEvents=()=>{w=!1,r({call:"subscribe",events:Object.keys(i)})}})();const v=document.getElementById("token"),g=document.getElementById("close");g!=null&&(g.onclick=()=>(q(),!1));const u=document.getElementById("version");u!=null&&(u.onclick=()=>(T(),!1));const p=document.location.protocol+"//"+document.location.host,h=document.getElementById("link");h!=null&&h.setAttribute("href",p);function b(t){v!=null&&(v.setAttribute("href",p+"/#/pred/"+t),v.innerText="解析")}function m(t){t==1055?O():k()}function k(){const t=document.getElementById("all");t!=null&&t.classList.add("hide")}function O(){const t=document.getElementById("all");t!=null&&t.classList.remove("hide")}function q(){const t=document.getElementById("all");t!=null&&t.classList.toggle("hide")}const B=new I(0);let c=B.region;const S=["6.4","6.4","6.3"],y=[88,88,80];function T(){c=(c+1)%y.length,E(),x()}function E(){u!=null&&(u.innerText=S[c])}function x(){callOverlayHandler({call:"RequestMJIZoneState",packetLen:y[c]})}addOverlayListener("onMJICraftworkData",t=>{b(t.data)});addOverlayListener("onMJIZoneChanged",t=>{m(t.zoneID)});startOverlayEvents();E();callOverlayHandler({call:"RequestMJIZoneState",packetLen:y[c]}).then(t=>{const o=document.getElementById("not-inited");o!=null&&o.classList.add("hide"),O(),m(t.zoneID)});