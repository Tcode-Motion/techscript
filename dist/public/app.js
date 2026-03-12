// TechScript Web Auto-Generated Frontend
const $state = {};
function Component_Header(props) {
    var $el = document.createDocumentFragment();
    var child_el_0 = document.createElement('header');
    var $child_el_0 = document.createDocumentFragment();
    var child_child_el_0_0 = document.createElement('h1');
    child_child_el_0_0.innerText += 'TechScript Web Module';
    $child_el_0.appendChild(child_child_el_0_0);
    child_el_0.appendChild($child_el_0);
    $el.appendChild(child_el_0);
    return $el;
}
function Page_Home() {
    var $el = document.createDocumentFragment();
    var child_el_0 = document.createElement('div');
    var $child_el_0 = document.createDocumentFragment();
    var child_child_el_0_0 = Component_Header({});
    $child_el_0.appendChild(child_child_el_0_0);
    child_el_0.appendChild($child_el_0);
    var $child_el_0 = document.createDocumentFragment();
    var child_child_el_0_0 = document.createElement('p');
    child_child_el_0_0.innerText += 'Welcome to the future of web development with TechScript!';
    $child_el_0.appendChild(child_child_el_0_0);
    child_el_0.appendChild($child_el_0);
    var $child_el_0 = document.createDocumentFragment();
    var child_child_el_0_0 = document.createElement('button');
    child_child_el_0_0.innerText += 'Click me (Count: {click_count})';
    $child_el_0.appendChild(child_child_el_0_0);
    child_el_0.appendChild($child_el_0);
    $el.appendChild(child_el_0);
    return $el;
}

document.addEventListener('DOMContentLoaded', () => {
    const root = document.getElementById('app');
    root.innerHTML = '';
    root.appendChild(Page_Home());
});