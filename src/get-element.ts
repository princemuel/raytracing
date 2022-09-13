function hideContent(parent: ParentNode, content: string) {
  parent
    .querySelectorAll(content)!
    .forEach((item) => item.setAttribute('hidden', 'true'));
}

const showContent = (parent: ParentNode, content: string) => {
  parent.querySelector(content)!.removeAttribute('hidden');
};

type NodeOrDocument = Document | ParentNode;
/**
 * @author princemuel
 * @example
 * getElement<HTMLButtonElement>('.tabBtn', '.container')
 * getElement<HTMLButtonElement>('.tabBtn', '.container', false)
 * getElement<HTMLButtonElement>('.tabBtn', '.container', true)
 */

function getElement<T extends Element>(
  selector: string,
  scope: NodeOrDocument
): T;
function getElement<T extends Element>(
  selector: string,
  scope: NodeOrDocument,
  isElementArray: true
): T[];
function getElement<T extends Element>(
  selector: string,
  scope: NodeOrDocument,
  isElementArray: false
): T;
function getElement<T extends Element>(
  selector: string,
  scope: NodeOrDocument,
  isElementArray?: boolean
): T | T[] {
  try {
    if (isElementArray) {
      const element = [...scope.querySelectorAll(selector)] as T[];
      if (element.length < 1) throw Error;
      return element;
    } else {
      const element = scope.querySelector(selector) as T;
      if (!element) throw Error;
      return element;
    }
  } catch (e) {
    throw new Error(
      `There is an error. Check if the selector "${selector}" is correct.`
    );
  }
}

export { getElement, showContent, hideContent };
