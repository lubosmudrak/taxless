export class Homepage {

    /**
     * Object with the list of endpoints available from the home page.
     * @returns {} object
     */
    static requests(){
        return {
            login: "/prihlasenie"
        }
    }

    /**
     * Object with text variables visible on the home page.
     * @returns {} object
     */
    static variables(){
        return {
            title: "Vitaj v TAXLESS!"
        }
    }

    //getters

    /**
     * Get the login button on the home page.
     * @returns {Cypress.Chainable<JQuery<HTMLButtonElement>>} button HTML element
     */
    static getLoginButton(): Cypress.Chainable<JQuery<HTMLButtonElement>> {
        return cy.get(":nth-child(2) > .text-decoration-none > .card > .card-body > .card-title");
    }

    // clickers

    /**
     * perform clicking on the Login button via the user interface
     */
    static login(): void{
        cy.intercept("GET",this.requests().login).as("loginRequest");
        this.getLoginButton().click();
        cy.wait("@loginRequest");
    }
}
