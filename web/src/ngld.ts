import "./ngld/common.js";

let element = document.getElementById('token');
let clostBtn = document.getElementById('close');
if (clostBtn != null) {
  clostBtn.onclick = () => {
    toggleText();
    return false;
  };
}

let basePath = document.location.protocol + "//" + document.location.host;
let link = document.getElementById('link');
if (link != null) {
  link.setAttribute('href', basePath);
}

function setContent(content: string) {
  if (element != null) {
    element.setAttribute('href', basePath + "/#/pred/" + content)
    element.innerText = "解析";
  }
}

function onZoneChange(zone: number) {
  if (zone == 1055) {
    showText();
  } else {
    hideText();
  }
}

function hideText() {
  let container = document.getElementById("all");
  if (container != null) {
    container.classList.add("hide");
  }
}

function showText() {
  let container = document.getElementById("all");
  if (container != null) {
    container.classList.remove("hide");
  }
}

function toggleText() {
  let container = document.getElementById("all");
  if (container != null) {
    container.classList.toggle("hide");
  }
}

// 添加数据处理
addOverlayListener('onMJICraftworkData', (data) => {
  setContent(data.data);
});

addOverlayListener('onMJIZoneChanged', (data) => {
  onZoneChange(data.zoneID);
});

// 注册完毕，启动悬浮窗事件
startOverlayEvents();

callOverlayHandler({ call: "RequestMJIZoneState" }).then((val) => {
  let ele = document.getElementById("not-inited");
  if (ele != null) {
    ele.classList.add("hide");
  }
  showText();
  onZoneChange(val.zoneID);
});
