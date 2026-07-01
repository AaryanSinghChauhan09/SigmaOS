/** sigma-auth-guard — PQC authentication guard component */
export class SigmaAuthGuard extends HTMLElement {
  connectedCallback() { this.innerHTML = '<div class="sigma-auth-guard">AuthGuard</div>'; }
}
customElements.define('sigma-auth-guard', SigmaAuthGuard);
