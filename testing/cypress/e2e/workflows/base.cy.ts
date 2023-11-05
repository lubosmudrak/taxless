import { page_names } from "../../support/page_names";

let pageNames = new page_names();

describe("W_B: base workflows", () => {
    beforeEach(() => {
        //TODO: cleanup routine
    })

    it("W_B_TC00: framework test", () => {
        cy.visit(pageNames.home);
    });

});