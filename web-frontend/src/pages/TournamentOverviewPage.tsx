import { Card, Container, Form, Table } from 'react-bootstrap';
import DashboardLayout from '../components/DashboardLayout';
import { Loader, WidgetWrapper, Link, Section } from '@innexgo/common-react-components';
import ManageTournamentData from '../components/ManageTournamentData';
import ErrorMessage from '../components/ErrorMessage';

import update from 'immutability-helper';

import { unwrap, getFirstOr } from '@innexgo/frontend-common';

import format from "date-fns/format";

import { Async, AsyncProps } from 'react-async';
import { TournamentData, tournamentDataView, TournamentMembership, tournamentMembershipView, TournamentSubmission, tournamentSubmissionView, TournamentYear, TournamentYearDemand, tournamentYearDemandView, tournamentYearView } from '../utils/api';
import { ApiKey } from '@innexgo/frontend-auth-api';
import { AuthenticatedComponentProps } from '@innexgo/auth-react-components';
import ManageTournamentSubmissionOverview from '../components/ManageTournamentSubmissionOverview';
import CompeteTournamentManager from '../components/CompeteTournamentManager';

type ManageTournamentPageData = {
  tournamentData: TournamentData,
  tournamentSubmissions: TournamentSubmission[],
  tournamentYears: TournamentYear[],
  tournamentYearDemands: TournamentYearDemand[],
  myMembership?: TournamentMembership;
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

  const myMembership: TournamentMembership | undefined = await tournamentMembershipView({
    tournamentId: [props.tournamentId],
    creatorUserId: [props.apiKey.creatorUserId],
    apiKey: props.apiKey.key,
    onlyRecent: true,
  })
    .then(unwrap)
    .then(x => x[0]);

  return {
    tournamentData,
    tournamentYears,
    tournamentYearDemands,
    tournamentSubmissions,
    myMembership
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
              <Section name="Overview" id="overview">
                <ManageTournamentSubmissionOverview
                  tournamentSubmissions={data.tournamentSubmissions}
                  apiKey={props.apiKey}
                  tournamentData={data.tournamentData}
                  tournamentYears={data.tournamentYears}
                  tournamentYearDemands={data.tournamentYearDemands}
                  adminView={false}
                />
              </Section>
              {data.myMembership !== undefined && data.myMembership.active
                ? <Section name="Compete" id="compete">
                  <CompeteTournamentManager
                    tournamentData={data.tournamentData}
                    tournamentYearDemands={data.tournamentYearDemands}
                    postCreate={ts => setData(update(data, { tournamentSubmissions: { $push: [ts] } }))}
                    apiKey={props.apiKey}
                  />
                </Section>
                : null
              }
            </>}
            </Async.Fulfilled>
          </>}
        </Async>
      </Container>
    </DashboardLayout >
  )
}


export default ManageTournamentPage;
