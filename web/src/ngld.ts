import { Config } from "./model/config.js";
import "./ngld/common.js";

const element = document.getElementById('token');
const clostBtn = document.getElementById('close');
if (clostBtn != null) {
  clostBtn.onclick = () => {
    toggleText();
    return false;
  };
}
const versionBtn = document.getElementById('version');
if (versionBtn != null) {
  versionBtn.onclick = () => {
    toggleVersion();
    return false;
  }
}

const basePath = document.location.protocol + "//" + document.location.host;
const link = document.getElementById('link');
if (link != null) {
  link.setAttribute('href', basePath);
}

function setContent(content: string) {
  console.log("craftwork data:", content);
  if (element != null) {
    element.setAttribute('href', basePath + "/#/pred/" + content);
    element.innerText = "解析";
  }
}

function onZoneChange(zone: number) {
  console.log("zone change:", zone);
  if (zone == 1055) {
    showText();
  } else {
    hideText();
  }
}

function hideText() {
  const container = document.getElementById("all");
  if (container != null) {
    container.classList.add("hide");
  }
}

function showText() {
  const container = document.getElementById("all");
  if (container != null) {
    container.classList.remove("hide");
  }
}

function toggleText() {
  const container = document.getElementById("all");
  if (container != null) {
    container.classList.toggle("hide");
  }
}

const config = new Config(0);
let version = config.region;

const versionText = ["7.1", "7.1", "7.1"];
const packetLen = [96, 96, 96];

function toggleVersion() {
  version = (version + 1) % packetLen.length;
  updateVersionText();
  setVersion();
}

function updateVersionText() {
  if (versionBtn != null) {
    versionBtn.innerText = versionText[version];
  }
}

function setVersion() {
  callOverlayHandler({
    call: "RequestMJIZoneState",
    packetLen: packetLen[version]
  });
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
updateVersionText();

callOverlayHandler({
  call: "RequestMJIZoneState",
  packetLen: packetLen[version]
}).then((val) => {
  const ele = document.getElementById("not-inited");
  if (ele != null) {
    ele.classList.add("hide");
  }
  showText();
  onZoneChange(val.zoneID);
});
