import React from "react"
import { Formik, FormikHelpers, FormikErrors, isNaN } from 'formik'
import { Button, Form } from "react-bootstrap";
import { TournamentData, TournamentMembership, tournamentMembershipNew } from "../utils/api";
import { isErr } from '@innexgo/frontend-common';
import { ApiKey } from '@innexgo/frontend-auth-api';
import { AuthenticatedComponentProps } from '@innexgo/auth-react-components';


type CreateTournamentMembershipProps = {
    apiKey: ApiKey,
    tournamentData: TournamentData,
    postSubmit: (t: TournamentMembership) => void,
}

function CreateTournamentMembership(props: CreateTournamentMembershipProps) {

    type CreateTournamentMembershipValue = {}

    const onSubmit = async (values: CreateTournamentMembershipValue,
        fprops: FormikHelpers<CreateTournamentMembershipValue>) => {

        const maybeTournament = await tournamentMembershipNew({
            tournamentId: props.tournamentData.tournament.tournamentId,
            active: true,
            apiKey: props.apiKey.key,
        });

        if (isErr(maybeTournament)) {
            switch (maybeTournament.Err) {
                case "UNAUTHORIZED": {
                    fprops.setStatus({
                        failureResult: "Not authorized to create tournament",
                        successResult: ""
                    });
                    break;
                }
                case "TOURNAMENT_STARTED": {
                    fprops.setStatus({
                        failureResult: "Tournament has already started",
                        successResult: ""
                    });
                    break;
                }
                default: {
                    fprops.setStatus({
                        failureResult: "An unknown or network error has occured while trying to create tournament.",
                        successResult: ""
                    });
                    break;
                }
            }
            return;
        }

        fprops.setStatus({
            failureResult: "",
            successResult: "Tournament Created"
        });
        // execute callback
        props.postSubmit(maybeTournament.Ok);
    }

    return <>
        <Formik<CreateTournamentMembershipValue>
            onSubmit={onSubmit}
            initialValues={{}}
            initialStatus={{
                failureResult: "",
                successResult: ""
            }}
        >
            {(fprops) => <>
                <Form
                    noValidate
                    onSubmit={fprops.handleSubmit} >
                    <div hidden={fprops.status.successResult !== ""}>
                        <Form.Group className="mb-3">
                            <Button type="submit">Submit Form</Button>
                        </Form.Group>
                        <Form.Text className="text-danger">{fprops.status.failureResult}</Form.Text>
                    </div>
                    <Form.Text className="text-success">{fprops.status.successResult}</Form.Text>
                </Form>
            </>}
        </Formik>
    </>
}

export default CreateTournamentMembership;
