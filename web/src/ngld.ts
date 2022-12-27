import "./ngld/common.js";

// 添加数据处理
addOverlayListener('onMJICraftworkData', (data) => {
  console.log(`MJI Craftwork DATA: ${JSON.stringify(data)}`);
  let element = document.getElementById('token');
  if (element != null) {
    element.innerText = data.data;
  }
});
// 注册完毕，启动悬浮窗事件
startOverlayEvents();
