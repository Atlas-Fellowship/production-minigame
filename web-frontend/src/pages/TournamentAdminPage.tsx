import { Card, Container, Form, Table } from 'react-bootstrap';
import DashboardLayout from '../components/DashboardLayout';
import { Loader, WidgetWrapper, Link, Section } from '@innexgo/common-react-components';
import ManageTournamentData from '../components/ManageTournamentData';
import ErrorMessage from '../components/ErrorMessage';

import update from 'immutability-helper';

import { unwrap, getFirstOr } from '@innexgo/frontend-common';

import format from "date-fns/format";

import { Async, AsyncProps } from 'react-async';
import { TournamentData, tournamentDataView, TournamentSubmission, tournamentSubmissionView, TournamentYear, TournamentYearDemand, tournamentYearDemandView, tournamentYearView } from '../utils/api';
import { ApiKey } from '@innexgo/frontend-auth-api';
import { AuthenticatedComponentProps } from '@innexgo/auth-react-components';
import ManageTournamentSubmissionOverview from '../components/ManageTournamentSubmissionOverview';

type ManageTournamentPageData = {
  tournamentData: TournamentData,
  tournamentSubmissions: TournamentSubmission[],
  tournamentYears: TournamentYear[],
  tournamentYearDemands: TournamentYearDemand[],
}

const loadManageTournamentPage = async (props: AsyncProps<ManageTournamentPageData>): Promise<ManageTournamentPageData> => {
  const tournamentData = await tournamentDataView({
    tournamentId: [props.tournamentId],
    onlyRecent: true,
    apiKey: props.apiKey.key
  })
    .then(unwrap)
    .then(x => getFirstOr(x, "NOT_FOUND"))
    .then(unwrap);

  // Really weak security protection, should be changed to serverside!
  if(tournamentData.creatorUserId !== props.apiKey.creatorUserId) {
      throw new Error("UNAUTHORIZED");
  }

  const tournamentSubmissions = await tournamentSubmissionView({
    tournamentId: [props.tournamentId],
    apiKey: props.apiKey.key
  })
    .then(unwrap);

  const tournamentYears = await tournamentYearView({
    tournamentId: [props.tournamentId],
    apiKey: props.apiKey.key,
    onlyRecent: false,
  })
    .then(unwrap);

  const tournamentYearDemands = await tournamentYearDemandView({
    tournamentId: [props.tournamentId],
    apiKey: props.apiKey.key,
  })
    .then(unwrap);

  return {
    tournamentData,
    tournamentYears,
    tournamentYearDemands,
    tournamentSubmissions,
  };
}

function ManageTournamentPage(props: AuthenticatedComponentProps) {
  const tournamentId = parseInt(new URLSearchParams(window.location.search).get("tournamentId") ?? "");

  return (
    <DashboardLayout {...props}>
      <Container fluid className="py-4 px-4">
        <Async promiseFn={loadManageTournamentPage} tournamentId={tournamentId} apiKey={props.apiKey}>{
          ({ setData }) => <>
            <Async.Pending><Loader /></Async.Pending>
            <Async.Rejected>{e => <ErrorMessage error={e} />}</Async.Rejected>
            <Async.Fulfilled<ManageTournamentPageData>>{data => <>
              <Section name="Tournament Data" id="intro">
                <div className="my-3">
                  <ManageTournamentData
                    setTournamentData={td => setData(update(data, { tournamentData: { $set: td } }))}
                    tournamentData={data.tournamentData}
                    apiKey={props.apiKey}
                  />
                </div>
              </Section>
              <Section name="Admin Overview" id="overview">
                <ManageTournamentSubmissionOverview
                  tournamentData={data.tournamentData}
                  tournamentYears={data.tournamentYears}
                  tournamentYearDemands={data.tournamentYearDemands}
                  tournamentSubmissions={data.tournamentSubmissions}
                  adminView={true}
                  onlyShowYou={false}
                  apiKey={props.apiKey}
                />
              </Section>
            </>}
            </Async.Fulfilled>
          </>}
        </Async>
      </Container>
    </DashboardLayout>
  )
}


export default ManageTournamentPage;
