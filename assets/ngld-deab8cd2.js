import{a as I}from"./config-986b78c7.js";(function(){let n=/[\?&]OVERLAY_WS=([^&]+)/.exec(location.href),o=null,s=[],L=0,d={},i={},r=null,w=!1;if(n){let t=function(){o=new WebSocket(n[1]),o.addEventListener("error",e=>{console.error(e)}),o.addEventListener("open",()=>{console.log("Connected!");let e=s;s=null;for(let l of e)r(l)}),o.addEventListener("message",e=>{try{e=JSON.parse(e.data)}catch{console.error("Invalid message received: ",e);return}e.rseq!==void 0&&d[e.rseq]?(d[e.rseq](e),delete d[e.rseq]):f(e)}),o.addEventListener("close",()=>{s=[],console.log("Trying to reconnect..."),setTimeout(()=>{t()},300)})};r=e=>{s?s.push(e):o.send(JSON.stringify(e))},t()}else{let t=function(){if(!window.OverlayPluginApi||!window.OverlayPluginApi.ready){setTimeout(t,300);return}let e=s;s=null,window.__OverlayCallback=f;for(let[l,a]of e)r(l,a)};r=(e,l)=>{s?s.push([e,l]):OverlayPluginApi.callHandler(JSON.stringify(e),l)},t()}function f(t){if(i[t.type])for(let e of i[t.type])e(t)}window.dispatchOverlayEvent=f,window.addOverlayListener=(t,e)=>{w&&i[t]&&console.warn(`A new listener for ${t} has been registered after event transmission has already begun.
Some events might have been missed and no cached values will be transmitted.
Please register your listeners before calling startOverlayEvents().`),i[t]||(i[t]=[]),i[t].push(e)},window.removeOverlayListener=(t,e)=>{if(i[t]){let l=i[t],a=l.indexOf(e);a>-1&&l.splice(a,1)}},window.callOverlayHandler=t=>{let e;return o?(t.rseq=L++,e=new Promise(l=>{d[t.rseq]=l}),r(t)):e=new Promise(l=>{r(t,a=>{l(a==null?null:JSON.parse(a))})}),e},window.startOverlayEvents=()=>{w=!1,r({call:"subscribe",events:Object.keys(i)})}})();const v=document.getElementById("token"),g=document.getElementById("close");g!=null&&(g.onclick=()=>(q(),!1));const u=document.getElementById("version");u!=null&&(u.onclick=()=>(T(),!1));const p=document.location.protocol+"//"+document.location.host,h=document.getElementById("link");h!=null&&h.setAttribute("href",p);function b(n){console.log("craftwork data:",n),v!=null&&(v.setAttribute("href",p+"/#/pred/"+n),v.innerText="解析")}function m(n){console.log("zone change:",n),n==1055?O():k()}function k(){const n=document.getElementById("all");n!=null&&n.classList.add("hide")}function O(){const n=document.getElementById("all");n!=null&&n.classList.remove("hide")}function q(){const n=document.getElementById("all");n!=null&&n.classList.toggle("hide")}const B=new I(0);let c=B.region;const S=["7.1","7.1","7.1"],y=[96,96,96];function T(){c=(c+1)%y.length,E(),x()}function E(){u!=null&&(u.innerText=S[c])}function x(){callOverlayHandler({call:"RequestMJIZoneState",packetLen:y[c]})}addOverlayListener("onMJICraftworkData",n=>{b(n.data)});addOverlayListener("onMJIZoneChanged",n=>{m(n.zoneID)});startOverlayEvents();E();callOverlayHandler({call:"RequestMJIZoneState",packetLen:y[c]}).then(n=>{const o=document.getElementById("not-inited");o!=null&&o.classList.add("hide"),O(),m(n.zoneID)});
