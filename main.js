document.addEventListener('DOMContentLoaded', () => {
    fetch('widgets')
    .then(response => response.json())
    .then(widgets => {
        const container = document.querySelector('.widgets');
        widgets.forEach(widget => {
            const widgetElement = document.createElement('div');
            widgetElement.className = 'widget';
            widgetElement.innerHTML = '<h2>${widget.title}</h2><p>${widget.content}</p>';
            container.appendChild(widgetElement);
        });
    });
});