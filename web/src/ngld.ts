import "./ngld/common.js";

let element = document.getElementById('token');

// 添加数据处理
addOverlayListener('onMJICraftworkData', (data) => {
  console.log(`MJI Craftwork DATA: ${JSON.stringify(data)}`);
  setContent(data.data);
});

// 注册完毕，启动悬浮窗事件
startOverlayEvents();

let clostBtn = document.getElementById('close');
if (clostBtn != null) {
  clostBtn.onclick = () => {
    if (element != null) {
      element.innerText = "";
    }
  };
}

let basePath = document.location.protocol + "//" + document.location.host;

function setContent(content: string) {
  if (element != null) {
    element.setAttribute('href', basePath + "/#/pred/" + content)
    element.innerText = "解析";
  }
}

let link = document.getElementById('link');
if (link != null) {
  link.setAttribute('href', basePath);
}
