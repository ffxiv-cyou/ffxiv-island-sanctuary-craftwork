declare function dispatchOverlayEvent(msg: any);
declare function addOverlayListener(event: string, cb: (msg: any) => void);
declare function removeOverlayListener(event: string, cb: (msg: any) => void);
declare function callOverlayHandler(msg: any);

declare function startOverlayEvents();
