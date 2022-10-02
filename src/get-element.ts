type Parent = Document | ParentNode | string;
type NodeOrDocument = Document | ParentNode;

/**
 * @author princemuel
 * @example
 * getElement<HTMLButtonElement>('.tabBtn', '.container')
 * getElement<HTMLButtonElement>('.tabBtn', document)
 * getElement<HTMLButtonElement>('.tabBtn', document, false)
 * getElement<HTMLButtonElement>('.tabBtn', navContainer, false)
 * getElement<HTMLButtonElement>('.tabBtn', '.container', true)
 */

function getElement<T extends Element>(selector: string, scope: Parent): T;
function getElement<T extends Element>(
  selector: string,
  scope: Parent,
  isElementArray: false
): T;
function getElement<T extends Element>(
  selector: string,
  scope: Parent,
  isElementArray: true
): T[];
function getElement<T extends Element>(
  selector: string,
  scope: Parent,
  isElementArray?: boolean
): T | T[] {
  try {
    const node = getScope(scope);
    if (isElementArray) {
      const element = [...node.querySelectorAll<T>(selector)];
      if (element.length < 1) throw Error;
      return element;
    } else {
      const element = node.querySelector<T>(selector);
      if (!element) throw Error;
      return element;
    }
  } catch (e) {
    console.log(e);
    throw new Error(
      `There is an error. Check if the selector "${selector}" is correct.`
    );
  }
}

function getScope(node: Parent | string) {
  try {
    if (typeof node === 'string') {
      return document.querySelector(node) as NodeOrDocument;
    }
    return node;
  } catch (error) {
    console.log(error);
    throw new Error(
      `There is an error. Check if the selector "${node}" is correct.`
    );
  }
}

export { getElement };
