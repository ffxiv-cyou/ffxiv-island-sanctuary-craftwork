(function(){"use strict";let r;const u=new Array(128).fill(void 0);u.push(void 0,null,!0,!1);function L(e){return u[e]}let E=u.length;function H(e){e<132||(u[e]=E,E=e)}function N(e){const n=L(e);return H(e),n}const S=typeof TextDecoder<"u"?new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw Error("TextDecoder not available")}};typeof TextDecoder<"u"&&S.decode();let F=null;function M(){return(F===null||F.byteLength===0)&&(F=new Uint8Array(r.memory.buffer)),F}function C(e,n){return e=e>>>0,S.decode(M().subarray(e,e+n))}let R=null;function D(){return(R===null||R.byteLength===0)&&(R=new Uint16Array(r.memory.buffer)),R}let c=0;function P(e,n){const t=n(e.length*2,2)>>>0;return D().set(e,t/2),c=e.length,t}function d(e,n){const t=n(e.length*1,1)>>>0;return M().set(e,t/1),c=e.length,t}function A(e,n){if(!(e instanceof n))throw new Error(`expected instance of ${n.name}`);return e.ptr}let W=null;function l(){return(W===null||W.byteLength===0)&&(W=new Int32Array(r.memory.buffer)),W}function v(e,n){return e=e>>>0,D().subarray(e/2,e/2+n)}function V(e,n){return e=e>>>0,M().subarray(e/1,e/1+n)}function $(e,n){try{const o=r.__wbindgen_add_to_stack_pointer(-16),a=d(e,r.__wbindgen_malloc),i=c;r.pattern_predict(o,a,i,n);var t=l()[o/4+0],_=l()[o/4+1],s=V(t,_).slice();return r.__wbindgen_free(t,_*1,1),s}finally{r.__wbindgen_add_to_stack_pointer(16)}}function G(e,n){try{const o=r.__wbindgen_add_to_stack_pointer(-16),a=d(e,r.__wbindgen_malloc),i=c;r.pattern_predict_adv(o,a,i,n);var t=l()[o/4+0],_=l()[o/4+1],s=v(t,_).slice();return r.__wbindgen_free(t,_*2,2),s}finally{r.__wbindgen_add_to_stack_pointer(16)}}let I=null;function J(){return(I===null||I.byteLength===0)&&(I=new Int8Array(r.memory.buffer)),I}function K(e,n){return e=e>>>0,J().subarray(e/1,e/1+n)}function Q(e,n){try{const o=r.__wbindgen_add_to_stack_pointer(-16),a=d(e,r.__wbindgen_malloc),i=c;r.pattern_demand(o,a,i,n);var t=l()[o/4+0],_=l()[o/4+1],s=K(t,_).slice();return r.__wbindgen_free(t,_*1,1),s}finally{r.__wbindgen_add_to_stack_pointer(16)}}function X(e,n,t){const _=P(e,r.__wbindgen_malloc),s=c,o=d(n,r.__wbindgen_malloc),a=c,i=r.init_api_v2(_,s,o,a,t);return z.__wrap(i)}function Y(e){E===u.length&&u.push(u.length+1);const n=E;return E=u[n],u[n]=e,n}const T=typeof TextEncoder<"u"?new TextEncoder("utf-8"):{encode:()=>{throw Error("TextEncoder not available")}},Z=typeof T.encodeInto=="function"?function(e,n){return T.encodeInto(e,n)}:function(e,n){const t=T.encode(e);return n.set(t),{read:e.length,written:t.length}};function ee(e,n,t){if(t===void 0){const i=T.encode(e),f=n(i.length,1)>>>0;return M().subarray(f,f+i.length).set(i),c=i.length,f}let _=e.length,s=n(_,1)>>>0;const o=M();let a=0;for(;a<_;a++){const i=e.charCodeAt(a);if(i>127)break;o[s+a]=i}if(a!==_){a!==0&&(e=e.slice(a)),s=t(s,_,_=a+e.length*3,1)>>>0;const i=M().subarray(s+a,s+_),f=Z(e,i);a+=f.written,s=t(s,_,a,1)>>>0}return c=a,s}const q=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>r.__wbg_apiv2_free(e>>>0));class z{static __wrap(n){n=n>>>0;const t=Object.create(z.prototype);return t.__wbg_ptr=n,q.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,q.unregister(this),n}free(){const n=this.__destroy_into_raw();r.__wbg_apiv2_free(n)}set_pattern(n){r.apiv2_set_pattern(this.__wbg_ptr,n)}simulate(n,t,_){try{const i=r.__wbindgen_add_to_stack_pointer(-16);A(n,h);const f=d(t,r.__wbindgen_malloc),w=c,p=d(_,r.__wbindgen_malloc),b=c;r.apiv2_simulate(i,this.__wbg_ptr,n.__wbg_ptr,f,w,p,b);var s=l()[i/4+0],o=l()[i/4+1],a=v(s,o).slice();return r.__wbindgen_free(s,o*2,2),a}finally{r.__wbindgen_add_to_stack_pointer(16)}}solve_day_with_batch(n,t,_,s,o,a,i,f){try{const g=r.__wbindgen_add_to_stack_pointer(-16);A(n,h);const y=d(_,r.__wbindgen_malloc),m=c,k=d(s,r.__wbindgen_malloc),U=c,j=d(o,r.__wbindgen_malloc),O=c;r.apiv2_solve_day_with_batch(g,this.__wbg_ptr,n.__wbg_ptr,t,y,m,k,U,j,O,a,i,f);var w=l()[g/4+0],p=l()[g/4+1],b=v(w,p).slice();return r.__wbindgen_free(w,p*2,2),b}finally{r.__wbindgen_add_to_stack_pointer(16)}}solve_day_dual(n,t,_,s,o,a,i){try{const b=r.__wbindgen_add_to_stack_pointer(-16);A(n,h);const g=d(_,r.__wbindgen_malloc),y=c,m=d(s,r.__wbindgen_malloc),k=c;r.apiv2_solve_day_dual(b,this.__wbg_ptr,n.__wbg_ptr,t,g,y,m,k,o,a,i);var f=l()[b/4+0],w=l()[b/4+1],p=v(f,w).slice();return r.__wbindgen_free(f,w*2,2),p}finally{r.__wbindgen_add_to_stack_pointer(16)}}solve_day_with_favor(n,t,_,s,o,a,i,f){try{const g=r.__wbindgen_add_to_stack_pointer(-16);A(n,h);const y=d(_,r.__wbindgen_malloc),m=c,k=d(s,r.__wbindgen_malloc),U=c,j=d(o,r.__wbindgen_malloc),O=c,oe=d(a,r.__wbindgen_malloc),ae=c;r.apiv2_solve_day_with_favor(g,this.__wbg_ptr,n.__wbg_ptr,t,y,m,k,U,j,O,oe,ae,i,f);var w=l()[g/4+0],p=l()[g/4+1],b=v(w,p).slice();return r.__wbindgen_free(w,p*2,2),b}finally{r.__wbindgen_add_to_stack_pointer(16)}}solve_week_single(n,t,_,s,o,a){try{const p=r.__wbindgen_add_to_stack_pointer(-16);A(n,h);const b=d(_,r.__wbindgen_malloc),g=c,y=d(a,r.__wbindgen_malloc),m=c;r.apiv2_solve_week_single(p,this.__wbg_ptr,n.__wbg_ptr,t,b,g,s,o,y,m);var i=l()[p/4+0],f=l()[p/4+1],w=v(i,f).slice();return r.__wbindgen_free(i,f*2,2),w}finally{r.__wbindgen_add_to_stack_pointer(16)}}solver_clear_cache(){r.apiv2_solver_clear_cache(this.__wbg_ptr)}solve_week_part(n,t,_,s,o,a,i){try{const b=r.__wbindgen_add_to_stack_pointer(-16);A(n,h);const g=d(_,r.__wbindgen_malloc),y=c,m=d(a,r.__wbindgen_malloc),k=c;r.apiv2_solve_week_part(b,this.__wbg_ptr,n.__wbg_ptr,t,g,y,s,o,m,k,i);var f=l()[b/4+0],w=l()[b/4+1],p=v(f,w).slice();return r.__wbindgen_free(f,w*2,2),p}finally{r.__wbindgen_add_to_stack_pointer(16)}}}const te=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>r.__wbg_craftworkinfo_free(e>>>0));class h{__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,te.unregister(this),n}free(){const n=this.__destroy_into_raw();r.__wbg_craftworkinfo_free(n)}get tension(){return r.__wbg_get_craftworkinfo_tension(this.__wbg_ptr)}set tension(n){r.__wbg_set_craftworkinfo_tension(this.__wbg_ptr,n)}get max_tension(){return r.__wbg_get_craftworkinfo_max_tension(this.__wbg_ptr)}set max_tension(n){r.__wbg_set_craftworkinfo_max_tension(this.__wbg_ptr,n)}get level(){return r.__wbg_get_craftworkinfo_level(this.__wbg_ptr)}set level(n){r.__wbg_set_craftworkinfo_level(this.__wbg_ptr,n)}get workers(){return r.__wbg_get_craftworkinfo_workers(this.__wbg_ptr)}set workers(n){r.__wbg_set_craftworkinfo_workers(this.__wbg_ptr,n)}constructor(n,t,_,s){const o=r.craftworkinfo_new(n,t,_,s);return this.__wbg_ptr=o>>>0,this}}typeof FinalizationRegistry>"u"||new FinalizationRegistry(e=>r.__wbg_favor_free(e>>>0)),typeof FinalizationRegistry>"u"||new FinalizationRegistry(e=>r.__wbg_gamedatarepo_free(e>>>0));async function ne(e,n){if(typeof Response=="function"&&e instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(e,n)}catch(_){if(e.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",_);else throw _}const t=await e.arrayBuffer();return await WebAssembly.instantiate(t,n)}else{const t=await WebAssembly.instantiate(e,n);return t instanceof WebAssembly.Instance?{instance:t,module:e}:t}}function re(){const e={};return e.wbg={},e.wbg.__wbindgen_object_drop_ref=function(n){N(n)},e.wbg.__wbg_new_abda76e883ba8a5f=function(){const n=new Error;return Y(n)},e.wbg.__wbg_stack_658279fe44541cf6=function(n,t){const _=L(t).stack,s=ee(_,r.__wbindgen_malloc,r.__wbindgen_realloc),o=c;l()[n/4+1]=o,l()[n/4+0]=s},e.wbg.__wbg_error_f851667af71bcfc6=function(n,t){let _,s;try{_=n,s=t,console.error(C(n,t))}finally{r.__wbindgen_free(_,s,1)}},e.wbg.__wbindgen_throw=function(n,t){throw new Error(C(n,t))},e}function _e(e,n){return r=e.exports,B.__wbindgen_wasm_module=n,W=null,I=null,R=null,F=null,r}async function B(e){if(r!==void 0)return r;typeof e>"u"&&(e="/assets/mji_craftwork_bg.wasm");const n=re();(typeof e=="string"||typeof Request=="function"&&e instanceof Request||typeof URL=="function"&&e instanceof URL)&&(e=fetch(e));const{instance:t,module:_}=await ne(await e,n);return _e(t,_)}function x(e){return new h(e.tension,e.max_tension,e.level,e.workers)}async function se(){await B(),self.postMessage({type:"inited"});let e;self.onmessage=async n=>{const t=n.data;switch(t.type){case"init_repo":e=X(t.recipe,t.pops,t.pop_row),self.postMessage({type:"init_repo"});return;case"pattern_predict":{const _=$(t.array,t.demands);self.postMessage(_,[_.buffer]);return}case"pattern_predict_adv":{const _=G(t.array,t.demands);self.postMessage(_,[_.buffer]);return}case"pattern_demand":{const _=Q(t.array,t.day);self.postMessage(_,[_.buffer]);return}default:if(!e)return self.reportError("repo is not inited. "+t.type);break}switch(t.type){case"set_pattern":e.set_pattern(t.pattern),self.postMessage({type:"set_pattern"});break;case"solve_week":{const _=x(t.state),s=e.solve_week_single(_,t.level,t.ban_list,t.time,t.with_cost,t.pattern);self.postMessage(s,[s.buffer]);break}case"solve_multi_day":{const _=x(t.state),s=e.solve_day_with_batch(_,t.level,t.ban_list,t.set,t.demands,t.worker,t.time,t.with_cost);self.postMessage(s,[s.buffer]);break}case"solve_day_dual":{const _=x(t.state),s=e.solve_day_dual(_,t.level,t.ban_list,t.demands,t.worker,t.time,t.with_cost);self.postMessage(s,[s.buffer]);break}case"solve_day_favor":{const _=x(t.state),s=e.solve_day_with_favor(_,t.level,t.ban_list,t.set,t.demands,t.favors,t.time,t.with_cost);self.postMessage(s,[s.buffer]);break}case"solve_week_part":{const _=x(t.state),s=e.solve_week_part(_,t.level,t.ban_list,t.time,t.with_cost,t.pattern,t.part_id);self.postMessage(s,[s.buffer]);break}case"solve_cache_clear":{e.solver_clear_cache(),self.postMessage({type:"solve_cache_clear"});break}case"simulate_multi":{const _=x(t.state),s=e.simulate(_,t.seq,t.demands);self.postMessage(s,[s.buffer]);break}case"simulate":return self.reportError("simulate is deprecated. use simulate_multi instead");case"solve_day":return self.reportError("solve_day is deprecated. use solve_multi_day instead");default:console.log("unexpected type",t.type),self.reportError("unexpected type "+t.type);break}}}se()})();