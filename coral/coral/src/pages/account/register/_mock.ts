// eslint-disable-next-line import/no-extraneous-dependencies
import type { Request, Response } from 'express';

export default {
  'POST  /api/v1/user/register': (_: Request, res: Response) => {
    res.send({
      data: { status: 'ok', currentAuthority: 'user' },
    });
  },
};
