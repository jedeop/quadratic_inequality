(()=>{var e,r,t,n,o,a,i,s={},c={};function p(e){var r=c[e];if(void 0!==r)return r.exports;var t=c[e]={id:e,loaded:!1,exports:{}};return s[e](t,t.exports,p),t.loaded=!0,t.exports}p.m=s,e="function"==typeof Symbol?Symbol("webpack then"):"__webpack_then__",r="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",t=e=>{e&&(e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},n=e=>!--e.r&&e(),o=(e,r)=>e?e.push(r):n(r),p.a=(a,i,s)=>{var c,p,l,u=s&&[],d=a.exports,f=!0,b=!1,m=(r,t,n)=>{b||(b=!0,t.r+=r.length,r.map(((r,o)=>r[e](t,n))),b=!1)},h=new Promise(((e,r)=>{l=r,p=()=>(e(d),t(u),u=0)}));h[r]=d,h[e]=(e,r)=>{if(f)return n(e);c&&m(c,e,r),o(u,e),h.catch(r)},a.exports=h,i((a=>{if(!a)return p();var i,s;c=(a=>a.map((a=>{if(null!==a&&"object"==typeof a){if(a[e])return a;if(a.then){var i=[];a.then((e=>{s[r]=e,t(i),i=0}));var s={};return s[e]=(e,r)=>(o(i,e),a.catch(r)),s}}var c={};return c[e]=e=>n(e),c[r]=a,c})))(a);var l=new Promise(((e,t)=>{(i=()=>e(s=c.map((e=>e[r])))).r=0,m(c,i,t)}));return i.r?l:s})).then(p,l),f=!1},p.d=(e,r)=>{for(var t in r)p.o(r,t)&&!p.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:r[t]})},p.f={},p.e=e=>Promise.all(Object.keys(p.f).reduce(((r,t)=>(p.f[t](e,r),r)),[])),p.u=e=>e+".bootstrap.js",p.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),p.hmd=e=>((e=Object.create(e)).children||(e.children=[]),Object.defineProperty(e,"exports",{enumerable:!0,set:()=>{throw new Error("ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: "+e.id)}}),e),p.o=(e,r)=>Object.prototype.hasOwnProperty.call(e,r),a={},i="create-wasm-app:",p.l=(e,r,t,n)=>{if(a[e])a[e].push(r);else{var o,s;if(void 0!==t)for(var c=document.getElementsByTagName("script"),l=0;l<c.length;l++){var u=c[l];if(u.getAttribute("src")==e||u.getAttribute("data-webpack")==i+t){o=u;break}}o||(s=!0,(o=document.createElement("script")).charset="utf-8",o.timeout=120,p.nc&&o.setAttribute("nonce",p.nc),o.setAttribute("data-webpack",i+t),o.src=e),a[e]=[r];var d=(r,t)=>{o.onerror=o.onload=null,clearTimeout(f);var n=a[e];if(delete a[e],o.parentNode&&o.parentNode.removeChild(o),n&&n.forEach((e=>e(t))),r)return r(t)},f=setTimeout(d.bind(null,void 0,{type:"timeout",target:o}),12e4);o.onerror=d.bind(null,o.onerror),o.onload=d.bind(null,o.onload),s&&document.head.appendChild(o)}},p.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},p.v=(e,r,t,n)=>{var o=fetch(p.p+""+t+".module.wasm");return"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(o,n).then((r=>Object.assign(e,r.instance.exports))):o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((r=>Object.assign(e,r.instance.exports)))},(()=>{var e;p.g.importScripts&&(e=p.g.location+"");var r=p.g.document;if(!e&&r&&(r.currentScript&&(e=r.currentScript.src),!e)){var t=r.getElementsByTagName("script");t.length&&(e=t[t.length-1].src)}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),p.p=e})(),(()=>{var e={179:0};p.f.j=(r,t)=>{var n=p.o(e,r)?e[r]:void 0;if(0!==n)if(n)t.push(n[2]);else{var o=new Promise(((t,o)=>n=e[r]=[t,o]));t.push(n[2]=o);var a=p.p+p.u(r),i=new Error;p.l(a,(t=>{if(p.o(e,r)&&(0!==(n=e[r])&&(e[r]=void 0),n)){var o=t&&("load"===t.type?"missing":t.type),a=t&&t.target&&t.target.src;i.message="Loading chunk "+r+" failed.\n("+o+": "+a+")",i.name="ChunkLoadError",i.type=o,i.request=a,n[1](i)}}),"chunk-"+r,r)}};var r=(r,t)=>{var n,o,[a,i,s]=t,c=0;if(a.some((r=>0!==e[r]))){for(n in i)p.o(i,n)&&(p.m[n]=i[n]);s&&s(p)}for(r&&r(t);c<a.length;c++)o=a[c],p.o(e,o)&&e[o]&&e[o][0](),e[o]=0},t=self.webpackChunkcreate_wasm_app=self.webpackChunkcreate_wasm_app||[];t.forEach(r.bind(null,0)),t.push=r.bind(null,t.push.bind(t))})(),p.e(10).then(p.bind(p,10)).catch((e=>console.error("Error importing `index.js`:",e)))})();