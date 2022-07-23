import { ApiKey } from "@innexgo/frontend-auth-api"
import { Formik, FormikErrors, FormikHelpers } from "formik"
import { Button, Form } from "react-bootstrap"
import { TournamentData, TournamentSubmission, tournamentSubmissionNew } from "../utils/api"
import { isErr, unwrap } from '@innexgo/frontend-common';

import { Prism as SyntaxHighligher } from 'react-syntax-highlighter';
import { a11yDark } from 'react-syntax-highlighter/dist/esm/styles/prism';


type CreateTournamentSubmissionProps = {
  tournamentData: TournamentData,
  apiKey: ApiKey,
  postSubmit: (ts: TournamentSubmission) => void
}



function CreateTournamentSubmission(props: CreateTournamentSubmissionProps) {

  type CreateSubmissionValue = {
    amount: number,
  }

  const onSubmit = async (values: CreateSubmissionValue,
    fprops: FormikHelpers<CreateSubmissionValue>) => {

    let errors: FormikErrors<CreateSubmissionValue> = {};

    // Validate input
    let hasError = false;

    if (typeof values.amount === "string" || isNaN(values.amount)) {
      errors.amount = "Please enter the amount you wish to produce.";
      hasError = true;
    }
    if(values.amount < 0) {
      errors.amount = "The amount you wish to produce must be positive";
      hasError = true;
    }

    fprops.setErrors(errors);
    if (hasError) {
      return;
    }

    const maybeTournamentSubmission = await tournamentSubmissionNew({
      tournamentId: props.tournamentData.tournament.tournamentId,
      amount: values.amount,
      apiKey: props.apiKey.key,
    });


    if (isErr(maybeTournamentSubmission)) {
      switch (maybeTournamentSubmission.Err) {
        case "TOURNAMENT_ARCHIVED": {
          fprops.setStatus({
            failureResult: "This tournament has been archived",
            successResult: ""
          });
          break;
        }
        default: {
          fprops.setStatus({
            failureResult: "An unknown or network error has occured while trying to create submission.",
            successResult: ""
          });
          break;
        }
      }
      return;
    }

    fprops.setStatus({
      failureResult: "",
      successResult: "Submission Created"
    });
    // execute callback
    props.postSubmit(maybeTournamentSubmission.Ok);
  }

  return <>
    <Formik<CreateSubmissionValue>
      onSubmit={onSubmit}
      initialValues={{
        amount: 0,
      }}
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
              <Form.Label>Amount to Produce</Form.Label>
              <Form.Control
                type="number"
                onChange={fprops.handleChange}
                value={fprops.values.amount}
                name="amount"
                isInvalid={!!fprops.errors.amount}
              />
              <Form.Control.Feedback type="invalid">{fprops.errors.amount}</Form.Control.Feedback>
            </Form.Group>
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

export default CreateTournamentSubmission;
