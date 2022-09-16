import { getElement } from './get-element';

const primaryHeader = getElement<HTMLElement>('.primary-header', document);
const navToggle = getElement(
  '.mobile-nav-toggle',
  document
) as HTMLButtonElement;
const primaryNav = getElement('.primary-navigation', document) as HTMLElement;
const slide = getElement('.slider', document) as HTMLUListElement;
const timeElement = getElement('[data-time]', document) as HTMLTimeElement;

const year = new Date().getFullYear().toString();

timeElement.dateTime = year;
timeElement.innerText = year;

navToggle.addEventListener('click', () => {
  primaryNav.hasAttribute('data-visible')
    ? navToggle.setAttribute('aria-expanded', 'false')
    : navToggle.setAttribute('aria-expanded', 'true');
  primaryNav.toggleAttribute('data-visible');
  primaryHeader.toggleAttribute('data-overlay');
});

// @ts-expect-error
const slider = new A11YSlider(slide, {
  adaptiveHeight: false,
  dots: true,
  centerMode: true,
  arrows: false,
  responsive: {
    480: {
      dots: false, // dots enabled 1280px and up
    },
  },
});
