import { page_names } from "../../support/page_names";

let pageNames = new page_names();

describe("M_B: base module", () => {
    beforeEach(() => {
        //TODO: cleanup routine
    })

    it("M_B_TC00: framework test", () => {
        cy.visit(pageNames.home);
    });

});