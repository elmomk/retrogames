(function() {
  var head = document.head;

  if (!head.querySelector('link[rel="manifest"]')) {
    var m = document.createElement('link');
    m.rel = 'manifest';
    m.href = '/manifest.json';
    head.appendChild(m);
  }

  if (!head.querySelector('meta[name="theme-color"]')) {
    var tc = document.createElement('meta');
    tc.name = 'theme-color';
    tc.content = '#ff2d78';
    head.appendChild(tc);
  }

  if (!head.querySelector('link[rel="apple-touch-icon"]')) {
    var ai = document.createElement('link');
    ai.rel = 'apple-touch-icon';
    ai.href = '/icons/icon-192.png';
    head.appendChild(ai);
  }

  if (!head.querySelector('meta[name="apple-mobile-web-app-capable"]')) {
    var awac = document.createElement('meta');
    awac.name = 'apple-mobile-web-app-capable';
    awac.content = 'yes';
    head.appendChild(awac);
  }

  if (!head.querySelector('meta[name="apple-mobile-web-app-status-bar-style"]')) {
    var asb = document.createElement('meta');
    asb.name = 'apple-mobile-web-app-status-bar-style';
    asb.content = 'black-translucent';
    head.appendChild(asb);
  }
})();

if ('serviceWorker' in navigator) {
  window.addEventListener('load', function() {
    navigator.serviceWorker.register('/sw.js', { scope: '/' });
  });
}
